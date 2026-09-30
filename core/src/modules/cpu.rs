use crate::{Info, Module};

pub struct Cpu;

impl Module for Cpu {
    fn name(&self) -> &'static str {
        "cpu"
    }

    fn detect(&self) -> Option<Info> {
        let content = std::fs::read_to_string("/proc/cpuinfo").ok()?;
        let value = parse_cpuinfo(&content)?;
        Some(Info::new("CPU", value))
    }
}

pub fn parse_cpuinfo(content: &str) -> Option<String> {
    let mut model_name: Option<String> = None;
    let mut core_count: usize = 0;

    for line in content.lines() {
        if let Some((key, val)) = line.split_once(':') {
            match key.trim() {
                "model name" if model_name.is_none() => {
                    let cleaned = val.split_whitespace().collect::<Vec<_>>().join(" ");
                    if !cleaned.is_empty() {
                        model_name = Some(cleaned);
                    }
                }
                "processor" => core_count += 1,
                _ => {}
            }
        }
    }

    let model = model_name?;
    if core_count == 0 {
        return None;
    }

    Some(format!("{model} ({core_count})"))
}

pub fn parse_cpu(content: &str) -> Option<String> {
    parse_cpuinfo(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    /// Realistic Fedora Linux /proc/cpuinfo fixture with 2 processor blocks (AMD Ryzen 7 5800X).
    const FEDORA_RYZEN_CPUINFO: &str = r#"processor	: 0
vendor_id	: AuthenticAMD
cpu family	: 25
model		: 33
model name	: AMD Ryzen 7 5800X
stepping	: 0
microcode	: 0xa201016
cpu MHz		: 3800.000
cache size	: 512 KB
physical id	: 0
siblings	: 16
core id		: 0
cpu cores	: 8
apicid		: 0
initial apicid	: 0
fpu		: yes
fpu_exception	: yes
cpuid level	: 16
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba ibrs ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif v_spec_ctrl umip pku ospke vaes vpclmulqdq rdpid overflow_recov succor smca fsrm
bugs		: sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips	: 7586.12
TLB size	: 2560 4K pages
clflush size	: 64
cache_alignment	: 64
address sizes	: 48 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate cpb eff_freq_ro [13] [14]

processor	: 1
vendor_id	: AuthenticAMD
cpu family	: 25
model		: 33
model name	: AMD Ryzen 7 5800X
stepping	: 0
microcode	: 0xa201016
cpu MHz		: 3800.000
cache size	: 512 KB
physical id	: 0
siblings	: 16
core id		: 1
cpu cores	: 8
apicid		: 2
initial apicid	: 2
fpu		: yes
fpu_exception	: yes
cpuid level	: 16
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba ibrs ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif v_spec_ctrl umip pku ospke vaes vpclmulqdq rdpid overflow_recov succor smca fsrm
bugs		: sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips	: 7586.12
TLB size	: 2560 4K pages
clflush size	: 64
cache_alignment	: 64
address sizes	: 48 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate cpb eff_freq_ro [13] [14]
"#;

    #[test]
    fn test_parse_cpuinfo_fedora_2_cores() {
        let result = parse_cpuinfo(FEDORA_RYZEN_CPUINFO);
        assert_snapshot!(result.unwrap(), @"AMD Ryzen 7 5800X (2)");
    }

    #[test]
    fn test_parse_cpuinfo_16_cores() {
        let mut content = String::new();
        for i in 0..16 {
            content.push_str(&format!(
                "processor\t: {i}\nvendor_id\t: AuthenticAMD\nmodel name\t: AMD Ryzen 7 5800X\n\n"
            ));
        }
        let result = parse_cpuinfo(&content);
        assert_snapshot!(result.unwrap(), @"AMD Ryzen 7 5800X (16)");
    }

    #[test]
    fn test_parse_cpuinfo_empty_and_invalid() {
        assert_eq!(parse_cpuinfo(""), None);
        assert_eq!(parse_cpuinfo("unknown: field"), None);
        assert_eq!(parse_cpuinfo("processor: 0"), None);
        assert_eq!(parse_cpuinfo("model name: Intel Core i5"), None);
    }

    #[test]
    fn test_cpu_module_name() {
        let cpu = Cpu;
        assert_eq!(cpu.name(), "cpu");
    }

    #[test]
    fn test_cpu_detect() {
        let cpu = Cpu;
        if let Some(info) = cpu.detect() {
            assert_eq!(info.label, "CPU");
            assert!(info.value.contains('('));
            assert!(info.value.ends_with(')'));
        }
    }
}

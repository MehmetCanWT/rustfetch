use std::mem;

pub fn get_cell_size() -> Option<(u16, u16)> {
    unsafe {
        let mut ws: libc::winsize = mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0
            && ws.ws_col > 0
            && ws.ws_row > 0
            && ws.ws_xpixel > 0
            && ws.ws_ypixel > 0
        {
            return Some((ws.ws_xpixel / ws.ws_col, ws.ws_ypixel / ws.ws_row));
        }
    }
    None
}

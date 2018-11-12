pub struct IncomingHttpHeaders{

}
pub struct Request {
        http_version: String,
        http_mersion_major: u8,
        http_mersion_minor: u8,
        //connection: None,
        headers: IncomingHttpHeaders,
        //raw_headers: String[],
        //trailers: { [key: String]: String | undefined },
        //rawTrailers: String[],
        time_out: u64,
        method: String,
        url: String,
        status_code: u16,
        status_message: String,
        //socket: net.Socket
}

pub(crate) struct RequestMethods(String);

impl RequestMethods {
    pub const GET: &'static str = "GET / HTTP/1.1\r\n";
    pub const POST: &'static str = "POST / HTTP/1.1\r\n";
    pub const UNKNOWN: &'static str = "";

    pub(crate) fn get_request_type(request: &str) -> &str {
        if request.starts_with(RequestMethods::GET){
            return RequestMethods::GET;
        }

        if request.starts_with(RequestMethods::POST){
            return RequestMethods::POST;
        }

        RequestMethods::UNKNOWN
    }
}
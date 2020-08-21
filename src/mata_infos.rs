
struct Environ {
    REQUEST_METHOD: String,
    SCRIPT_NAME: String,
    PATH_INFO: String,
    QUERY_STRING: String,
    CONTENT_TYPE: Option<String>,
    CONTENT_LENGTH: Option<String>,
    SERVER_NAME: String,
    SERVER_PORT: String,
    SERVER_PROTOCOL: String,
}

use http::Method;

header! {
    /// `Access-Control-Request-Method` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-request-method-request-header)
    ///
    /// The `Access-Control-Request-Method` header indicates which method will be
    /// used in the actual request as part of the preflight request.
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Request-Method: \"Access-Control-Request-Method\" \":\" Method
    /// ```
    ///
    /// # Example values
    /// * `GET`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate headers;
    /// extern crate http;
    /// use headers::{Headers, AccessControlRequestMethod};
    /// use http::Method;
    ///
    /// let mut headers = Headers::new();
    /// headers.set(AccessControlRequestMethod(Method::GET));
    /// ```
    (AccessControlRequestMethod, "Access-Control-Request-Method") => [Method]

    test_access_control_request_method {
        test_header!(test1, vec![b"GET"]);
    }
}

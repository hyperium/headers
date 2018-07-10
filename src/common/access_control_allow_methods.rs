use http::Method;

header! {
    /// `Access-Control-Allow-Methods` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-allow-methods-response-header)
    ///
    /// The `Access-Control-Allow-Methods` header indicates, as part of the
    /// response to a preflight request, which methods can be used during the
    /// actual request.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Allow-Methods: "Access-Control-Allow-Methods" ":" #Method
    /// ```
    ///
    /// # Example values
    /// * `PUT, DELETE, XMODIFY`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate headers;
    /// extern crate http;
    /// use headers::{Headers, AccessControlAllowMethods};
    /// use http::Method;
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AccessControlAllowMethods(vec![Method::GET])
    /// );
    /// ```
    ///
    /// ```
    /// # extern crate headers;
    /// extern crate http;
    /// use headers::{Headers, AccessControlAllowMethods};
    /// use http::Method;
    /// use std::str::FromStr;
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     AccessControlAllowMethods(vec![
    ///         Method::GET,
    ///         Method::POST,
    ///         Method::PATCH,
    ///         Method::from_str("COPY").unwrap(),
    ///     ])
    /// );
    /// ```
    (AccessControlAllowMethods, "Access-Control-Allow-Methods") => (Method)*

    test_access_control_allow_methods {
        test_header!(test1, vec![b"PUT, DELETE, XMODIFY"]);
    }
}

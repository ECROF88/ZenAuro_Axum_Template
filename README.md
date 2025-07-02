1. middleware::from_fn
`req.extensions_mut().insert(claims);`
`Extension(claims): Extension<Claims>, // 使用 Extension 提取器`
2. use FromRequestParts trait
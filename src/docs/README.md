# API Documentation

This folder stores API documentation assets.

## Recommended: OpenAPI / Swagger

Add `utoipa` + `utoipa-swagger-ui` to `Cargo.toml` to generate interactive
docs automatically from your handler annotations.

```toml
utoipa = { version = "4", features = ["axum_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "6", features = ["axum"] }
```

Then mount the UI in `bootstrap/app.rs`:

```rust
use utoipa_swagger_ui::SwaggerUi;

let router = router
    .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()));
```

## Available Endpoints (Starter)

| Method | Path                    | Description          | Auth Required |
|--------|-------------------------|----------------------|---------------|
| POST   | /api/v1/auth/login      | Login                | No            |
| POST   | /api/v1/auth/refresh    | Refresh access token | No            |
| POST   | /api/v1/auth/logout     | Logout               | Yes           |
| POST   | /api/v1/users           | Create user          | No            |
| GET    | /api/v1/users           | List users           | Yes           |
| GET    | /api/v1/users/:id       | Get user by ID       | Yes           |
| PATCH  | /api/v1/users/:id       | Update user          | Yes           |
| DELETE | /api/v1/users/:id       | Delete user          | Yes (Admin)   |

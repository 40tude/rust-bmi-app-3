# Deployment Guide

This document provides step-by-step instructions for deploying the BMI Calculator application to Heroku.

## Prerequisites

1. **Heroku Account**: Sign up at [heroku.com](https://heroku.com)
2. **Heroku CLI**: Install from [devcenter.heroku.com/articles/heroku-cli](https://devcenter.heroku.com/articles/heroku-cli)
3. **Git**: Ensure git is installed and project is in a git repository

## Deployment Steps

### 1. Login to Heroku

```bash
heroku login
```

This opens a browser window for authentication.

### 2. Create Heroku Application

```bash
heroku create bmi-calculator-rust
```

Replace `bmi-calculator-rust` with your preferred app name. If the name is taken, Heroku will suggest an alternative.

### 3. Set Buildpack

The Rust buildpack is already configured in `.buildpacks`, but you can explicitly set it:

```bash
heroku buildpacks:set emk/rust
```

### 4. Verify Configuration

Check that all deployment files are present:

```bash
ls Procfile RustConfig rust-toolchain.toml .buildpacks
```

All files should exist.

### 5. Deploy Application

```bash
git push heroku main
```

Or if your main branch is named `master`:

```bash
git push heroku master
```

### 6. Monitor Build

Heroku will compile your Rust application. This takes 3-5 minutes on first deploy. Watch the build log for any errors.

### 7. Open Application

Once deployed successfully:

```bash
heroku open
```

This opens your application in the default browser.

### 8. View Logs

Monitor application logs:

```bash
heroku logs --tail
```

Press `Ctrl+C` to stop tailing logs.

## Configuration

### Environment Variables

The application automatically uses Heroku's `PORT` environment variable. No manual configuration needed.

To set custom environment variables:

```bash
heroku config:set VARIABLE_NAME=value
```

### Scaling

By default, Heroku runs one web dyno. To scale:

```bash
heroku ps:scale web=2
```

This runs 2 instances (requires paid dyno).

## Troubleshooting

### Build Failures

**Problem**: Compilation errors during deployment

**Solution**:
1. Ensure local build succeeds: `cargo build --release`
2. Check Rust version matches: `rustc --version`
3. Review build logs: `heroku logs --tail`

### Application Crashes

**Problem**: Application starts but immediately crashes

**Solution**:
1. Check logs: `heroku logs --tail`
2. Verify Procfile command: `web: ./target/release/bmi_calculator`
3. Ensure binary name matches Cargo.toml package name

### Port Binding Errors

**Problem**: Application fails to bind to port

**Solution**:
The application automatically reads `PORT` env var. Verify code:
```rust
let port = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .unwrap_or(3000);
```

### Memory Issues

**Problem**: Application exceeds memory limits (512MB on free tier)

**Solution**:
1. Monitor memory: `heroku ps`
2. Upgrade dyno type: `heroku ps:type hobby` (paid)
3. Optimize code: reduce allocations, use streaming

## Maintenance

### Viewing Application Info

```bash
heroku info
```

### Restarting Application

```bash
heroku restart
```

### Rolling Back

If a deployment fails, rollback to previous release:

```bash
heroku rollback
```

### Database Integration (Future)

To add PostgreSQL:

```bash
heroku addons:create heroku-postgresql:mini
```

Then access via `DATABASE_URL` environment variable.

## Custom Domain (Optional)

### Add Domain

```bash
heroku domains:add www.example.com
```

### Configure DNS

Add CNAME record pointing to:
```
your-app-name.herokuapp.com
```

### SSL Certificate

Heroku provides automatic SSL certificates for custom domains on paid dynos.

## Performance Tips

1. **Use Release Builds**: Procfile uses `./target/release/bmi_calculator` (optimized)
2. **Enable LTO**: Already configured in `Cargo.toml`
3. **Mimalloc Allocator**: Already configured for 15-25% performance boost
4. **Keep Dyno Warm**: Free dynos sleep after 30 minutes of inactivity

## Cost Estimate

- **Free Tier**: 1 web dyno, sleeps after 30 min inactivity
- **Hobby Tier**: $7/month, never sleeps, custom SSL
- **Production Tier**: $25-$500/month, autoscaling, metrics

## Security Considerations

1. **HTTPS**: Enabled by default on `*.herokuapp.com`
2. **Environment Variables**: Use `heroku config:set` for secrets
3. **CORS**: Currently permissive, restrict in production:

```rust
let cors = CorsLayer::new()
    .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap());
```

## Monitoring

### View Metrics

```bash
heroku logs --tail | grep "bmi.calculation"
```

Filters structured logs for BMI calculations.

### Add-ons

Consider adding monitoring tools:

```bash
heroku addons:create papertrail:choklad  # Log management
heroku addons:create newrelic:wayne      # Performance monitoring
```

## Cleanup

### Stop Application

```bash
heroku ps:scale web=0
```

### Delete Application

```bash
heroku apps:destroy --app bmi-calculator-rust
```

Enter app name to confirm deletion.

## Next Steps

1. **CI/CD**: Set up GitHub Actions for automatic deployment
2. **Testing**: Add integration tests before deployment
3. **Monitoring**: Implement health check endpoint
4. **Analytics**: Track BMI calculation patterns
5. **Features**: Add BMI history, user accounts, charts

## Resources

- [Heroku Rust Support](https://devcenter.heroku.com/articles/getting-started-with-rust)
- [Rust Buildpack](https://github.com/emk/heroku-buildpack-rust)
- [Heroku CLI Commands](https://devcenter.heroku.com/articles/heroku-cli-commands)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Tokio Documentation](https://docs.rs/tokio/latest/tokio/)

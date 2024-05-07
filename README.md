# From zero to production in Rust

Check the Justfile for the commands to run the project.

## TODO:

- [ ] Subscription confirmation should not be a GET request, but a POST request: https://github.com/LukeMathWalker/zero-to-production/issues/187.

### Section 7

- [ ] What happens if a user tries to subscribe twice? Make sure that they receive two confirmation emails;
- [ ] What happens if a user clicks on a confirmation link twice?
- [ ] What happens if the subscription token is well-formatted but non-existent?
- [ ] Add validation on the incoming token, we are currently passing the raw user input straight into a query (thanks sqlx for protecting us from SQL injections <3);
- [ ] Use a proper templating solution for our emails (e.g. tera);
- [ ] Anything that comes to your mind!

### Section 10
- [ ] OWASP’s provides a minimum set of requirements when it comes to password strength - passwords should be longer than 12 characters but shorter than 129 characters.
Add these validation checks to our POST /admin/password endpoint.
- [X] Add a "Send a newsletter issue" link to the admin dashboard
- [ ] Add an HTML form at GET /admin/newsletters to submit a new issue;
- [ ] Adapt POST /newsletters to process the form data:
    - Change the route to POST /admin/newsletters;
    - Migrate from ‘Basic’ to session-based authentication;
    - Use the Form extractor (application/x-www-form-urlencoded) instead of the Json extractor (application/json) to handle the request body
    - Adapt the tests
- [ ] OAuth

### Section 11
- [ ] There is no retry if the email delivery attempt fails. We could enhance this by adding a `n_retries` and `execute_after` columns to keep track of how many attemps have already taken place and when the next attempt should be executed.
- [ ] Add an exponential backoff with jitter in the `issue_delivery_worker::worker_loop` function.
- [ ] Add an expiry mechanism for the idempotency keys using background workers as reference.

## Troubleshooting

### Too many open files when running tests

On MacOS, you may encounter the error `Too many open files` when running tests. This is because the default limit of open file descriptors is too low. You can increase the limit by running the following command:

```sh
ulimit -n 2048
```

Reference: https://github.com/WeareJH/config-gen/issues/44

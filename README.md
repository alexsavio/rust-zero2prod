# From zero to production in Rust

Check the Justfile for the commands to run the project.

## TODO:

### Section 7

- [ ] What happens if a user tries to subscribe twice? Make sure that they receive two confirmation emails;
- [ ] What happens if a user clicks on a confirmation link twice?
- [ ] What happens if the subscription token is well-formatted but non-existent?
- [ ] Add validation on the incoming token, we are currently passing the raw user input straight into a query (thanks sqlx for protecting us from SQL injections <3);
- [ ] Use a proper templating solution for our emails (e.g. tera);
- [ ] Anything that comes to your mind!

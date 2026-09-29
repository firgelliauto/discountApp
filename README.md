# Bulk Quantity Discount
Reusable embedded Shopify app that creates a non-combinable automatic product discount powered by a Shopify Function.
## Rules
- Only products with the exact Shopify tag `Bulk Discount` qualify.
- 1–10 units: regular price.
- 11–49 units: 20% off.
- 50 or more units: 30% off.
- Quantity is evaluated per cart line.
- The automatic discount does not combine with product, order, or shipping discounts, so Shopify selects the best eligible discount for the customer.
## Local setup
1. Install Node.js 20.19+ and the Shopify CLI.
2. Install Rust with the `wasm32-wasip1` target, then run `npm install`.
3. Run `shopify app config link` and select or create the Partner Dashboard app.
4. Run `shopify app dev --store firgelliautomation.myshopify.com`.
5. Install the development app, open it in Shopify admin, and click **Enable discount**.
6. Add the `Bulk Discount` tag to each eligible product and test the three quantity boundaries in checkout.
## Deployment and installation on other stores
Run `shopify app deploy`, host the web process and database, then select a Partner Dashboard distribution method. Choose public distribution for general App Store installation or custom distribution for stores in the same Plus organization. Install the generated app link on each store and click **Enable discount** once per store.
## Production database
The starter uses SQLite for local development. Before multi-instance production hosting, switch Prisma to PostgreSQL and set `DATABASE_URL` so sessions and store discount IDs are shared reliably.

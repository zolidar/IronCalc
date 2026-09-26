# IronCalc nodejs bindings

Zolidar publishes this package to the public **npm** registry as `@zolidar/ironcalc-nodejs`. Installing it needs no token or `.npmrc` setup.

## Installation

```bash
pnpm add @zolidar/ironcalc-nodejs
# or: npm install @zolidar/ironcalc-nodejs
```

If an `.npmrc` routes `@zolidar` to `npm.pkg.github.com`, remove that line so the package resolves from npm.

## Publishing (maintainers)

Run the **Publish npm packages** workflow (`.github/workflows/npm.yml`) on `main` with `publish: true`. It authenticates with npm trusted publishing (GitHub Actions OIDC), so no npm token is stored in the repo. Each package gets provenance.

Every package, the main one and each `npm/*` platform package, needs its own trusted publisher. The `check-npm-trust` job fails the run if any is missing. To add one, for example for a new platform package after its first publish, log in to npm as an owner of the `@zolidar` org (2FA required) and run this in an interactive terminal with npm 11.15 or later:

```bash
npx -y npm@11 trust github @zolidar/ironcalc-nodejs-<platform> \
  --file npm.yml --repo zolidar/IronCalc --allow-publish -y
```

npm only accepts a trusted publisher for a package that already exists. Its first version has to be published by an owner some other way.

## Example usage

```javascript
import { Model } from '@zolidar/ironcalc-nodejs';

const model = new Model("Workbook1", "en", "UTC", "en");

model.setUserInput(0, 1, 1, "=1+1");

const result1 = model.getFormattedCellValue(0, 1, 1);
console.log('Cell value', result1); // "#ERROR"

model.evaluate();

const resultAfterEvaluate = model.getFormattedCellValue(0, 1, 1);
console.log('Cell value', resultAfterEvaluate); // 2

let result2 = model.getCellStyle(0, 1, 1);
console.log('Cell style', result2);
```

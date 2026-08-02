# IronCalc nodejs bindings

Zolidar publishes this package to **GitHub Packages** as `@zolidar/ironcalc-nodejs` (private).

## Installation (Zolidar eng)

1. Open **1Password → Engineering vault → `Github Zolidar Packages Read PAT`** and copy the token.
2. Export it (add to `~/.zshrc` if you want it in every shell):

```bash
export NODE_AUTH_TOKEN='ghp_…'   # paste from 1Password
```

3. In the consuming repo, ensure `.npmrc` contains:

```
@zolidar:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${NODE_AUTH_TOKEN}
```

(`svc-model-ts` already has this file.)

4. Install:

```bash
pnpm add @zolidar/ironcalc-nodejs
# or: npm install @zolidar/ironcalc-nodejs
```

Do **not** commit the token. A **401** usually means `NODE_AUTH_TOKEN` is unset or the PAT is not SSO-authorized for the **zolidar** org.

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

# IronCalc nodejs bindings

Zolidar publishes this package to the public **npm** registry as `@zolidar/ironcalc-nodejs`. Installing it needs no token or `.npmrc` setup.

## Installation

```bash
pnpm add @zolidar/ironcalc-nodejs
# or: npm install @zolidar/ironcalc-nodejs
```

If an `.npmrc` routes `@zolidar` to `npm.pkg.github.com`, remove that line so the package resolves from npm.

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

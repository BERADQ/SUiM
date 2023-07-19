const prettierConfigStandard = require('prettier-config-standard')

/** @type {import("prettier").Options} */
module.exports = {
  ...prettierConfigStandard,
  plugins: ['prettier-plugin-svelte']
}

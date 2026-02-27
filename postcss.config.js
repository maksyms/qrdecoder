module.exports = {
  plugins: [
    require('postcss-prefix-selector')({
      prefix: '#app',
      transform: function (prefix, selector, prefixedSelector) {
        if (selector.match(/^(html|body|:root)/)) {
          return prefix;
        }
        return prefixedSelector;
      }
    })
  ]
};

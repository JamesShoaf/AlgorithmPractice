const entityParser = (text) => {
  const dictionary = {
    '&quot;': '"',
    '&apos;': '\'',
    '&amp;': '&',
    '&gt;': '>',
    '&lt;': '<',
    '&frasl;': '/',
  };

  let parsed = '';
  let escaped = '';
  for (let i = 0; i < text.length; i += 1) {
    const char = text[i];
    if (escaped || char === '&') {
      if (char === '&') {
        parsed += escaped;
        escaped = '&';
      } else {
        escaped += char;
        if (dictionary[escaped]) {
          parsed += dictionary[escaped];
          escaped = '';
        }
      }
    } else {
      parsed += char;
    }
  }
  parsed += escaped;
  return parsed;
};

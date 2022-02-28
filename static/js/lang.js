let languagesElements = [],
  currentLang = "fr";

function getAllElementWithAttribute(attribute) {
  let matchingElement = [];
  let elements = document.getElementsByTagName("*");
  for (let i = 0; i < elements.length; i++) {
    let el = elements[i];
    if (el.getAttribute(attribute) !== null) {
      matchingElement.push(el);
    }
  }

  return matchingElement;
}

function findCurrentLang() {
  currentLang = "fr";
}

function currentLanguage() {
  return currentLang;
}

function updateLanguage(newLang) {
  if (newLang !== currentLang) {
    // Todo: get new language with api and change every languages elements
  }
}

document.addEventListener("DOMContentLoaded", () => {
  languagesElements = getAllElementWithAttribute("lg");
  findCurrentLang();
  console.log(languagesElements);
});

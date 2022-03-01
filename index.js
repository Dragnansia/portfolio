export function moveTo(el) {
  el = document.getElementById(el);
  el.scrollIntoView({
    block: "end",
    behavior: "smooth",
    inline: "center",
  });
}

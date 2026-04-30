(function () {
  "use strict";

  // ============================================================
  // Theme toggle
  // ============================================================

  const root = document.documentElement;
  const toggle = document.getElementById("theme-toggle");

  var prefersDark = matchMedia("(prefers-color-scheme: dark)");

  function setTheme(theme) {
    root.dataset.theme = theme;
    document.querySelector('meta[name="theme-color"]').content =
      theme === "dark" ? "#1a1510" : "#F5EDE0";
    toggle.textContent = theme === "dark" ? "3AM" : "BLINDS OPEN";
  }

  function systemTheme() {
    return prefersDark.matches ? "dark" : "light";
  }

  setTheme(systemTheme());

  prefersDark.addEventListener("change", function () {
    setTheme(systemTheme());
  });

  toggle.addEventListener("click", function () {
    setTheme(root.dataset.theme === "dark" ? "light" : "dark");
  });

  // ============================================================
  // Mobile nav toggle
  // ============================================================

  const hamburger = document.getElementById("hamburger");
  const navLinks = document.getElementById("nav-links");

  hamburger.addEventListener("click", function () {
    navLinks.classList.toggle("open");
  });

  // Close nav on link click (mobile)
  navLinks.querySelectorAll("a").forEach(function (link) {
    link.addEventListener("click", function () {
      navLinks.classList.remove("open");
    });
  });

  // ============================================================
  // Scroll reveal
  // ============================================================

  var reducedMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;

  if (!reducedMotion) {
    var observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) {
            entry.target.classList.add("revealed");
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.1 }
    );

    document.querySelectorAll(".reveal").forEach(function (el) {
      observer.observe(el);
    });
  } else {
    document.querySelectorAll(".reveal").forEach(function (el) {
      el.classList.add("revealed");
    });
  }

  // ============================================================
  // Tagline rotation
  // ============================================================

  var phrases = [
    "your eyes deserved this",
    "mostly warm, rarely blue",
    "contrast audited, morale pending",
    "one cool accent, because mud is not a palette",
    "less neon, more burnt toast",
    "the burnout is spreading",
    "deadlines stay, most of the blue glare goes away",
  ];
  var idx = 0;
  var taglineEl = document.querySelector("[data-tagline-rotate]");

  if (taglineEl) {
    setInterval(function () {
      taglineEl.style.opacity = "0";
      setTimeout(function () {
        idx = (idx + 1) % phrases.length;
        taglineEl.textContent = phrases[idx];
        taglineEl.style.opacity = "1";
      }, 400);
    }, 4000);
  }

  // ============================================================
  // Screenshot tabs
  // ============================================================

  var tabButtons = document.querySelectorAll("[data-tab]");
  var tabPanels = document.querySelectorAll("[data-panel]");

  tabButtons.forEach(function (btn) {
    btn.addEventListener("click", function () {
      var target = btn.dataset.tab;

      tabButtons.forEach(function (b) { b.classList.remove("active"); });
      tabPanels.forEach(function (p) { p.classList.remove("active"); });

      btn.classList.add("active");
      document.querySelector('[data-panel="' + target + '"]').classList.add("active");
    });
  });
})();

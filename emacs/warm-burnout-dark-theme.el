;;; warm-burnout-dark-theme.el --- Warm Burnout Dark theme -*- lexical-binding: t; -*-

;; Author: Felipe Lima, Chris Rose
;; URL: https://github.com/felipefdl/warm-burnout
;; Version: 1.0.0
;; Package-Requires: ((emacs "29.1"))
;; Keywords: faces themes

;;; Commentary:

;; Dark variant of Warm Burnout. Fully warm palette, minimal blue-spectrum
;; emission, WCAG AAA contrast (>= 7.0:1) for all syntax tokens.
;; Less blue, more therapy.

;;; Code:

(require 'warm-burnout)

(deftheme warm-burnout-dark
  "Warm Burnout Dark - consistent damage across all platforms.")

(warm-burnout--with-palette warm-burnout-dark-palette

  (custom-theme-set-faces
   'warm-burnout-dark

   ;; --- Core faces ---
   `(default ((t (:foreground ,fg :background ,bg))))
   `(cursor ((t (:background ,cursor))))
   `(region ((t (:background "#3a3520"))))
   `(highlight ((t (:background ,bg-highlight))))
   `(hl-line ((t (:background ,bg-highlight))))
   `(fringe ((t (:foreground ,fg-dim :background ,bg))))
   `(vertical-border ((t (:foreground ,border))))
   `(border ((t (:foreground ,border))))
   `(shadow ((t (:foreground ,fg-dim))))
   `(minibuffer-prompt ((t (:foreground ,func :weight bold))))
   `(link ((t (:foreground ,info :underline t))))
   `(link-visited ((t (:foreground ,number :underline t))))
   `(escape-glyph ((t (:foreground ,regex))))
   `(homoglyph ((t (:foreground ,regex))))
   `(match ((t (:background ,bg-search :weight bold))))
   `(trailing-whitespace ((t (:background ,error))))
   `(lazy-highlight ((t (:background ,bg-search))))

   ;; --- Line numbers ---
   `(line-number ((t (:foreground ,fg-gutter :background ,bg))))
   `(line-number-current-line ((t (:foreground ,fg :background ,bg-highlight :weight bold))))

   ;; --- Search ---
   `(isearch ((t (:background ,bg-search :foreground ,fg :weight bold))))
   `(isearch-fail ((t (:background ,error :foreground ,bg))))

   ;; --- Font-lock (syntax) ---
   `(font-lock-keyword-face ((t (:foreground ,keyword :weight bold))))
   `(font-lock-function-name-face ((t (:foreground ,func))))
   `(font-lock-function-call-face ((t (:foreground ,func))))
   `(font-lock-variable-name-face ((t (:foreground ,member))))
   `(font-lock-variable-use-face ((t (:foreground ,fg))))
   `(font-lock-type-face ((t (:foreground ,type :slant italic))))
   `(font-lock-string-face ((t (:foreground ,string))))
   `(font-lock-doc-face ((t (:foreground ,comment :slant italic))))
   `(font-lock-comment-face ((t (:foreground ,comment :slant italic))))
   `(font-lock-comment-delimiter-face ((t (:foreground ,comment :slant italic))))
   `(font-lock-constant-face ((t (:foreground ,constant))))
   `(font-lock-number-face ((t (:foreground ,number))))
   `(font-lock-builtin-face ((t (:foreground ,member))))
   `(font-lock-preprocessor-face ((t (:foreground ,keyword :weight bold))))
   `(font-lock-negation-char-face ((t (:foreground ,operator))))
   `(font-lock-regexp-grouping-backslash ((t (:foreground ,regex))))
   `(font-lock-regexp-grouping-construct ((t (:foreground ,regex))))
   `(font-lock-warning-face ((t (:foreground ,warn :weight bold))))
   `(font-lock-operator-face ((t (:foreground ,operator))))
   `(font-lock-property-name-face ((t (:foreground ,property))))
   `(font-lock-property-use-face ((t (:foreground ,property))))
   `(font-lock-escape-face ((t (:foreground ,regex))))
   `(font-lock-delimiter-face ((t (:foreground ,fg))))
   `(font-lock-bracket-face ((t (:foreground ,fg))))
   `(font-lock-misc-punctuation-face ((t (:foreground ,fg))))

   ;; --- Mode-line ---
   `(mode-line ((t (:foreground ,fg :background ,bg-float :box (:line-width 1 :color ,border)))))
   `(mode-line-inactive ((t (:foreground ,fg-dim :background ,bg-dim :box (:line-width 1 :color ,border)))))
   `(mode-line-emphasis ((t (:foreground ,keyword :weight bold))))
   `(mode-line-buffer-id ((t (:foreground ,func :weight bold))))

   ;; --- Header / tool / menu bars ---
   `(header-line ((t (:foreground ,fg :background ,bg-float :box (:line-width 1 :color ,border)))))
   `(header-line-highlight ((t (:foreground ,keyword :weight bold))))
   `(tool-bar ((t (:foreground ,fg :background ,bg-dim :box (:line-width 1 :color ,border)))))
   `(menu ((t (:foreground ,fg :background ,bg-dim))))
   `(tty-menu-enabled-face ((t (:foreground ,fg :background ,bg-dim))))
   `(tty-menu-disabled-face ((t (:foreground ,fg-dim :background ,bg-dim))))
   `(tty-menu-selected-face ((t (:foreground ,fg :background ,bg-highlight :weight bold))))

   ;; --- Completions ---
   `(completions-common-part ((t (:foreground ,func))))
   `(completions-first-difference ((t (:foreground ,keyword :weight bold))))

   ;; --- Dired ---
   `(dired-directory ((t (:foreground ,func :weight bold))))
   `(dired-symlink ((t (:foreground ,type :slant italic))))
   `(dired-ignored ((t (:foreground ,fg-dim))))

   ;; --- Errors, warnings ---
   `(error ((t (:foreground ,error))))
   `(warning ((t (:foreground ,warn))))
   `(success ((t (:foreground ,added))))

   ;; --- Diff ---
   `(diff-added ((t (:foreground ,added :background "#242918"))))
   `(diff-removed ((t (:foreground ,deleted :background "#341f1c"))))
   `(diff-changed ((t (:foreground ,modified :background "#24282d"))))
   `(diff-header ((t (:foreground ,func :weight bold))))
   `(diff-file-header ((t (:foreground ,keyword :weight bold))))
   `(diff-hunk-header ((t (:foreground ,type :slant italic))))

   ;; --- Magit ---
   `(magit-section-heading ((t (:foreground ,keyword :weight bold))))
   `(magit-section-highlight ((t (:background ,bg-highlight))))
   `(magit-diff-added ((t (:foreground ,added :background "#242918"))))
   `(magit-diff-added-highlight ((t (:foreground ,added :background "#2a351d"))))
   `(magit-diff-removed ((t (:foreground ,deleted :background "#341f1c"))))
   `(magit-diff-removed-highlight ((t (:foreground ,deleted :background "#422523"))))
   `(magit-diff-context ((t (:foreground ,fg-dim))))
   `(magit-diff-context-highlight ((t (:foreground ,fg-dim :background ,bg-highlight))))
   `(magit-diff-hunk-heading ((t (:foreground ,fg :background ,bg-float))))
   `(magit-diff-hunk-heading-highlight ((t (:foreground ,fg :background ,bg-float :weight bold))))
   `(magit-branch-local ((t (:foreground ,type))))
   `(magit-branch-remote ((t (:foreground ,added))))
   `(magit-branch-current ((t (:foreground ,type :box (:line-width 1 :color ,type)))))
   `(magit-hash ((t (:foreground ,fg-dim))))
   `(magit-log-author ((t (:foreground ,func))))
   `(magit-log-date ((t (:foreground ,comment))))
   `(magit-tag ((t (:foreground ,decorator :slant italic))))

   ;; --- Org-mode ---
   `(org-level-1 ((t (:foreground ,keyword :weight bold :height 1.1))))
   `(org-level-2 ((t (:foreground ,func :weight bold))))
   `(org-level-3 ((t (:foreground ,type :weight bold))))
   `(org-level-4 ((t (:foreground ,string :weight bold))))
   `(org-level-5 ((t (:foreground ,decorator :weight bold))))
   `(org-level-6 ((t (:foreground ,number :weight bold))))
   `(org-level-7 ((t (:foreground ,member :weight bold))))
   `(org-level-8 ((t (:foreground ,regex :weight bold))))
   `(org-block ((t (:background ,bg-dim))))
   `(org-block-begin-line ((t (:foreground ,fg-dim :background ,bg-dim))))
   `(org-block-end-line ((t (:foreground ,fg-dim :background ,bg-dim))))
   `(org-code ((t (:foreground ,string))))
   `(org-verbatim ((t (:foreground ,regex))))
   `(org-date ((t (:foreground ,type :underline t))))
   `(org-todo ((t (:foreground ,error :weight bold))))
   `(org-done ((t (:foreground ,added :weight bold))))
   `(org-headline-done ((t (:foreground ,fg-dim))))
   `(org-link ((t (:foreground ,info :underline t))))
   `(org-table ((t (:foreground ,fg))))
   `(org-formula ((t (:foreground ,number))))
   `(org-tag ((t (:foreground ,fg-dim :weight normal))))
   `(org-document-title ((t (:foreground ,keyword :weight bold :height 1.2))))
   `(org-document-info ((t (:foreground ,fg-dim))))

   ;; --- Markdown ---
   `(markdown-header-face-1 ((t (:foreground ,keyword :weight bold :height 1.1))))
   `(markdown-header-face-2 ((t (:foreground ,func :weight bold))))
   `(markdown-header-face-3 ((t (:foreground ,type :weight bold))))
   `(markdown-header-face-4 ((t (:foreground ,string :weight bold))))
   `(markdown-code-face ((t (:foreground ,string :background ,bg-dim))))
   `(markdown-inline-code-face ((t (:foreground ,string :background ,bg-dim))))
   `(markdown-link-face ((t (:foreground ,info :underline t))))
   `(markdown-url-face ((t (:foreground ,fg-dim :underline t))))
   `(markdown-bold-face ((t (:weight bold))))
   `(markdown-italic-face ((t (:slant italic))))

   ;; --- Company / Corfu ---
   `(company-tooltip ((t (:foreground ,fg :background ,bg-float))))
   `(company-tooltip-selection ((t (:background ,bg-highlight))))
   `(company-tooltip-common ((t (:foreground ,func :weight bold))))
   `(company-tooltip-annotation ((t (:foreground ,fg-dim))))
   `(company-scrollbar-bg ((t (:background ,bg-float))))
   `(company-scrollbar-fg ((t (:background ,fg-gutter))))
   `(corfu-default ((t (:foreground ,fg :background ,bg-float))))
   `(corfu-current ((t (:background ,bg-highlight))))
   `(corfu-bar ((t (:background ,fg-gutter))))

   ;; --- Vertico / Consult / Marginalia ---
   `(vertico-current ((t (:background ,bg-highlight))))
   `(consult-preview-match ((t (:background ,bg-search))))
   `(marginalia-documentation ((t (:foreground ,fg-dim :slant italic))))
   `(marginalia-key ((t (:foreground ,keyword))))
   `(marginalia-file-priv-dir ((t (:foreground ,type))))

   ;; --- Which-key ---
   `(which-key-key-face ((t (:foreground ,keyword :weight bold))))
   `(which-key-command-description-face ((t (:foreground ,fg))))
   `(which-key-group-description-face ((t (:foreground ,type))))
   `(which-key-separator-face ((t (:foreground ,fg-dim))))

   ;; --- Flycheck / Flymake ---
   `(flycheck-error ((t (:underline (:style wave :color ,error)))))
   `(flycheck-warning ((t (:underline (:style wave :color ,warn)))))
   `(flycheck-info ((t (:underline (:style wave :color ,info)))))
   `(flymake-error ((t (:underline (:style wave :color ,error)))))
   `(flymake-warning ((t (:underline (:style wave :color ,warn)))))
   `(flymake-note ((t (:underline (:style wave :color ,info)))))

   ;; --- LSP ---
   `(lsp-face-highlight-read ((t (:background ,bg-highlight))))
   `(lsp-face-highlight-write ((t (:background ,bg-highlight :weight bold))))
   `(lsp-face-highlight-textual ((t (:background ,bg-highlight))))

   ;; --- diff-hl ---
   `(diff-hl-insert ((t (:foreground ,added :background "#2f3f21"))))
   `(diff-hl-change ((t (:foreground ,modified :background "#303d4b"))))
   `(diff-hl-delete ((t (:foreground ,deleted :background "#502b2a"))))

   ;; --- Rainbow delimiters ---
   `(rainbow-delimiters-depth-1-face ((t (:foreground ,bracket1))))
   `(rainbow-delimiters-depth-2-face ((t (:foreground ,bracket2))))
   `(rainbow-delimiters-depth-3-face ((t (:foreground ,bracket3))))
   `(rainbow-delimiters-depth-4-face ((t (:foreground ,bracket4))))
   `(rainbow-delimiters-depth-5-face ((t (:foreground ,bracket5))))
   `(rainbow-delimiters-depth-6-face ((t (:foreground ,bracket6))))
   `(rainbow-delimiters-depth-7-face ((t (:foreground ,bracket1))))
   `(rainbow-delimiters-depth-8-face ((t (:foreground ,bracket2))))
   `(rainbow-delimiters-depth-9-face ((t (:foreground ,bracket3))))
   `(rainbow-delimiters-unmatched-face ((t (:foreground ,error :weight bold))))

   ;; --- Tree-sitter (uses font-lock faces, but some extras) ---
   `(tree-sitter-hl-face:function.call ((t (:foreground ,func))))
   `(tree-sitter-hl-face:method.call ((t (:foreground ,func))))
   `(tree-sitter-hl-face:property ((t (:foreground ,property))))
   `(tree-sitter-hl-face:tag ((t (:foreground ,tag :weight bold))))
   `(tree-sitter-hl-face:attribute ((t (:foreground ,decorator :slant italic))))
   `(tree-sitter-hl-face:operator ((t (:foreground ,operator))))
   `(tree-sitter-hl-face:punctuation ((t (:foreground ,fg))))
   `(tree-sitter-hl-face:punctuation.bracket ((t (:foreground ,fg))))

   ;; --- Show-paren ---
   `(show-paren-match ((t (:foreground ,cursor :background ,bg-highlight :weight bold))))
   `(show-paren-mismatch ((t (:foreground ,bg :background ,error :weight bold))))

   ;; --- Whitespace mode ---
   `(whitespace-space ((t (:foreground ,fg-gutter))))
   `(whitespace-tab ((t (:foreground ,fg-gutter))))
   `(whitespace-newline ((t (:foreground ,fg-gutter))))
   `(whitespace-trailing ((t (:background ,error))))
   `(whitespace-indentation ((t (:foreground ,fg-gutter))))

   ;; --- Tab-bar / Tab-line ---
   `(tab-bar ((t (:foreground ,fg-dim :background ,bg-dim))))
   `(tab-bar-tab ((t (:foreground ,fg :background ,bg :weight bold))))
   `(tab-bar-tab-inactive ((t (:foreground ,fg-dim :background ,bg-dim))))

   ;; --- Window divider ---
   `(window-divider ((t (:foreground ,border))))
   `(window-divider-first-pixel ((t (:foreground ,border))))
   `(window-divider-last-pixel ((t (:foreground ,border))))

   ;; --- Info / help ---
   `(info-title-1 ((t (:foreground ,keyword :weight bold :height 1.1))))
   `(info-title-2 ((t (:foreground ,func :weight bold))))
   `(info-title-3 ((t (:foreground ,type :weight bold))))
   `(info-node ((t (:foreground ,keyword :weight bold :slant italic))))

   ;; --- Compilation ---
   `(compilation-error ((t (:foreground ,error))))
   `(compilation-warning ((t (:foreground ,warn))))
   `(compilation-info ((t (:foreground ,info))))
   `(compilation-line-number ((t (:foreground ,fg-dim))))

   ;; --- Eshell / comint / term ---
   `(eshell-prompt ((t (:foreground ,keyword :weight bold))))
   `(eshell-ls-archive ((t (:foreground ,number))))
   `(eshell-ls-backup ((t (:foreground ,fg-dim))))
   `(eshell-ls-clutter ((t (:foreground ,fg-dim))))
   `(eshell-ls-directory ((t (:foreground ,func :weight bold))))
   `(eshell-ls-executable ((t (:foreground ,added))))
   `(eshell-ls-missing ((t (:foreground ,error))))
   `(eshell-ls-product ((t (:foreground ,fg-dim))))
   `(eshell-ls-readonly ((t (:foreground ,decorator))))
   `(eshell-ls-special ((t (:foreground ,member))))
   `(eshell-ls-symlink ((t (:foreground ,type :slant italic))))
   `(eshell-ls-unreadable ((t (:foreground ,error))))
   `(comint-highlight-prompt ((t (:foreground ,keyword :weight bold))))
   `(comint-highlight-input ((t (:foreground ,fg))))
   `(term-color-black ((t (:foreground ,bg-highlight :background ,bg-highlight))))
   `(term-color-red ((t (:foreground ,error :background ,error))))
   `(term-color-green ((t (:foreground ,added :background ,added))))
   `(term-color-yellow ((t (:foreground ,decorator :background ,decorator))))
   `(term-color-blue ((t (:foreground ,modified :background ,modified))))
   `(term-color-magenta ((t (:foreground ,number :background ,number))))
   `(term-color-cyan ((t (:foreground ,regex :background ,regex))))
   `(term-color-white ((t (:foreground ,fg :background ,fg))))

   ;; --- Eglot ---
   `(eglot-highlight-symbol-face ((t (:background ,bg-highlight))))
   `(eglot-mode-line ((t (:foreground ,func :weight bold))))
   `(eglot-diagnostic-tag-unnecessary-face ((t (:foreground ,fg-dim))))
   `(eglot-diagnostic-tag-deprecated-face ((t (:foreground ,fg-dim :strike-through t))))
   `(eglot-parameter-hint-face ((t (:foreground ,fg-dim :slant italic))))
   `(eglot-type-hint-face ((t (:foreground ,fg-dim :slant italic))))
   `(eglot-inlay-hint-face ((t (:foreground ,fg-dim :slant italic))))

   ;; --- Eldoc ---
   `(eldoc-highlight-function-argument ((t (:foreground ,keyword :weight bold))))

   ;; --- Ediff ---
   `(ediff-current-diff-A ((t (:background "#341f1c"))))
   `(ediff-current-diff-B ((t (:background "#242918"))))
   `(ediff-current-diff-C ((t (:background "#24282d"))))
   `(ediff-current-diff-Ancestor ((t (:background ,bg-highlight))))
   `(ediff-fine-diff-A ((t (:background "#502b2a" :weight bold))))
   `(ediff-fine-diff-B ((t (:background "#2a351d" :weight bold))))
   `(ediff-fine-diff-C ((t (:background "#303d4b" :weight bold))))
   `(ediff-fine-diff-Ancestor ((t (:background ,bg-highlight :weight bold))))
   `(ediff-even-diff-A ((t (:background ,bg-dim))))
   `(ediff-odd-diff-A ((t (:background ,bg-dim))))
   `(ediff-even-diff-B ((t (:background ,bg-dim))))
   `(ediff-odd-diff-B ((t (:background ,bg-dim))))
   `(ediff-even-diff-C ((t (:background ,bg-dim))))
   `(ediff-odd-diff-C ((t (:background ,bg-dim))))

   ;; --- Xref / pulse ---
   `(xref-match ((t (:background ,bg-search :weight bold))))
   `(xref-line-number ((t (:foreground ,fg-dim))))
   `(xref-file-header ((t (:foreground ,keyword :weight bold))))
   `(pulse-highlight-face ((t (:background ,bg-search))))
   `(pulse-highlight-start-face ((t (:background ,bg-search))))

   ;; --- Outline ---
   `(outline-1 ((t (:foreground ,keyword :weight bold))))
   `(outline-2 ((t (:foreground ,func :weight bold))))
   `(outline-3 ((t (:foreground ,type :weight bold))))
   `(outline-4 ((t (:foreground ,string :weight bold))))
   `(outline-5 ((t (:foreground ,decorator :weight bold))))
   `(outline-6 ((t (:foreground ,number :weight bold))))
   `(outline-7 ((t (:foreground ,member :weight bold))))
   `(outline-8 ((t (:foreground ,regex :weight bold))))

   ;; --- Buttons / widgets ---
   `(button ((t (:foreground ,info :underline t))))
   `(widget-button ((t (:foreground ,func :weight bold))))
   `(widget-button-pressed ((t (:foreground ,operator))))
   `(widget-field ((t (:foreground ,fg :background ,bg-dim))))
   `(widget-single-line-field ((t (:foreground ,fg :background ,bg-dim))))
   `(widget-inactive ((t (:foreground ,fg-dim))))
   `(widget-documentation ((t (:foreground ,fg-dim :slant italic))))

   ;; --- VC state ---
   `(vc-edited-state ((t (:foreground ,modified))))
   `(vc-locally-added-state ((t (:foreground ,added))))
   `(vc-needs-update-state ((t (:foreground ,warn))))
   `(vc-removed-state ((t (:foreground ,deleted))))
   `(vc-conflict-state ((t (:foreground ,error :weight bold))))
   `(vc-missing-state ((t (:foreground ,error))))
   `(vc-up-to-date-state ((t (:foreground ,fg-dim))))
   `(vc-state-base ((t (:foreground ,fg-dim))))

   ;; --- Customize buffers ---
   `(custom-button ((t (:foreground ,func :background ,bg-float
                                    :box (:line-width 1 :color ,border) :weight bold))))
   `(custom-button-mouse ((t (:foreground ,func :background ,bg-highlight
                                          :box (:line-width 1 :color ,func) :weight bold))))
   `(custom-button-pressed ((t (:foreground ,fg :background ,bg-highlight
                                            :box (:line-width 1 :color ,operator) :weight bold))))
   `(custom-button-unraised ((t (:foreground ,func :weight bold :underline t))))
   `(custom-button-pressed-unraised ((t (:foreground ,operator :weight bold :underline t))))
   `(custom-link ((t (:foreground ,info :underline t))))
   `(custom-state ((t (:foreground ,added))))
   `(custom-changed ((t (:foreground ,modified))))
   `(custom-modified ((t (:foreground ,modified))))
   `(custom-set ((t (:foreground ,added))))
   `(custom-saved ((t (:foreground ,added :underline t))))
   `(custom-themed ((t (:foreground ,decorator))))
   `(custom-invalid ((t (:foreground ,error :weight bold))))
   `(custom-rogue ((t (:foreground ,warn))))
   `(custom-comment ((t (:foreground ,comment :slant italic))))
   `(custom-comment-tag ((t (:foreground ,decorator :slant italic))))
   `(custom-variable-tag ((t (:foreground ,member :weight bold))))
   `(custom-variable-button ((t (:foreground ,func :weight bold :underline t))))
   `(custom-group-tag ((t (:foreground ,keyword :weight bold :height 1.1))))
   `(custom-group-tag-1 ((t (:foreground ,func :weight bold))))
   `(custom-face-tag ((t (:foreground ,type :weight bold :slant italic))))
   `(custom-visibility ((t (:foreground ,info :underline t))))

   ;; --- Misc built-ins ---
   `(secondary-selection ((t (:background ,bg-highlight))))
   `(tooltip ((t (:foreground ,fg :background ,bg-float))))
   `(Info-quoted ((t (:foreground ,string :inherit fixed-pitch)))))

  (custom-theme-set-variables
   'warm-burnout-dark
   `(ansi-color-names-vector
     [,bg ,error ,added ,keyword ,info ,number ,regex ,fg])))

;;;###autoload
(when load-file-name
  (add-to-list 'custom-theme-load-path
               (file-name-directory load-file-name)))

(provide-theme 'warm-burnout-dark)
;;; warm-burnout-dark-theme.el ends here

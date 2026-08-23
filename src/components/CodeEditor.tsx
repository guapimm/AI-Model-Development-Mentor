import { useEffect, useRef } from "react";
import { EditorState, Compartment } from "@codemirror/state";
import { EditorView, keymap, lineNumbers, highlightActiveLine } from "@codemirror/view";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import {
  defaultHighlightStyle,
  syntaxHighlighting,
  StreamLanguage,
  LanguageSupport,
} from "@codemirror/language";
import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
import { oneDark } from "@codemirror/theme-one-dark";
import { javascript } from "@codemirror/lang-javascript";
import { python } from "@codemirror/lang-python";
import { java } from "@codemirror/lang-java";
import { cpp } from "@codemirror/lang-cpp";
import { php } from "@codemirror/lang-php";
import { markdown } from "@codemirror/lang-markdown";
import { json } from "@codemirror/lang-json";
import { html } from "@codemirror/lang-html";
import { css } from "@codemirror/lang-css";
import { rust } from "@codemirror/legacy-modes/mode/rust";
import { ruby } from "@codemirror/legacy-modes/mode/ruby";
import { shell } from "@codemirror/legacy-modes/mode/shell";
import { yaml } from "@codemirror/legacy-modes/mode/yaml";
import { toml } from "@codemirror/legacy-modes/mode/toml";
import { go } from "@codemirror/legacy-modes/mode/go";
import { csharp } from "@codemirror/legacy-modes/mode/clike";
import { standardSQL } from "@codemirror/legacy-modes/mode/sql";

function rustLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(rust));
}

function rubyLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(ruby));
}

function shellLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(shell));
}

function yamlLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(yaml));
}

function tomlLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(toml));
}

function goLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(go));
}

function csharpLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(csharp));
}

function sqlLanguage(): LanguageSupport {
  return new LanguageSupport(StreamLanguage.define(standardSQL));
}

export function languageFor(lang: string | null | undefined) {
  switch (lang) {
    case "TypeScript":
      return javascript({ typescript: true });
    case "TSX":
      return javascript({ typescript: true, jsx: true });
    case "JavaScript":
      return javascript();
    case "JSX":
      return javascript({ jsx: true });
    case "Python":
      return python();
    case "Java":
      return java();
    case "C":
    case "C++":
      return cpp();
    case "C#":
      return csharpLanguage();
    case "PHP":
      return php();
    case "Rust":
      return rustLanguage();
    case "Go":
      return goLanguage();
    case "Ruby":
      return rubyLanguage();
    case "Markdown":
      return markdown();
    case "JSON":
      return json();
    case "HTML":
      return html();
    case "CSS":
    case "SCSS":
    case "Less":
      return css();
    case "SQL":
      return sqlLanguage();
    case "Shell":
    case "PowerShell":
    case "Batch":
      return shellLanguage();
    case "YAML":
      return yamlLanguage();
    case "TOML":
      return tomlLanguage();
    default:
      return null;
  }
}

export default function CodeEditor({
  content,
  language,
  readOnly,
  onChange,
  onSave,
}: {
  content: string;
  language: string | null | undefined;
  readOnly: boolean;
  onChange: (value: string) => void;
  onSave: () => void;
}) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  // Keep latest callbacks without re-creating the editor.
  const onChangeRef = useRef(onChange);
  const onSaveRef = useRef(onSave);
  onChangeRef.current = onChange;
  onSaveRef.current = onSave;

  useEffect(() => {
    if (!hostRef.current) return;

    const langCompartment = new Compartment();
    const lang = languageFor(language);

    const state = EditorState.create({
      doc: content,
      extensions: [
        lineNumbers(),
        history(),
        highlightActiveLine(),
        highlightSelectionMatches(),
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
        oneDark,
        keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap, indentWithTab]),
        langCompartment.of(lang ? [lang] : []),
        readOnly ? EditorState.readOnly.of(true) : EditorView.editable.of(true),
        EditorView.updateListener.of((update) => {
          if (update.docChanged && !readOnly) {
            onChangeRef.current(update.state.doc.toString());
          }
        }),
        EditorView.domEventHandlers({
          keydown: (event) => {
            if ((event.ctrlKey || event.metaKey) && event.key === "s") {
              event.preventDefault();
              onSaveRef.current();
              return true;
            }
            return false;
          },
        }),
      ],
    });

    const view = new EditorView({
      state,
      parent: hostRef.current,
    });

    return () => {
      view.destroy();
    };
    // Editor is created once per tab (parent uses `key`).
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return <div ref={hostRef} className="code-editor-host" />;
}

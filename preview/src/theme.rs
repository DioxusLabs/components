use dioxus::prelude::*;

const COOKIE_NAME: &str = "dx_theme";
const CHANNEL_NAME: &str = "dx-theme";

pub fn theme_seed() {
    _ = document::eval(&format!(
        r#"
        (function () {{
          if (window.__dx_theme_seeded) return;
          window.__dx_theme_seeded = true;

          const COOKIE_NAME = '{cookie_name}';
          const CHANNEL_NAME = '{channel_name}';

          function getCookie(name) {{
            const prefix = name + '=';
            const parts = document.cookie.split(';');
            for (let p of parts) {{
              p = p.trim();
              if (p.startsWith(prefix)) return decodeURIComponent(p.slice(prefix.length));
            }}
            return null;
          }}

          function apply(theme) {{
            if (theme === 'dark' || theme === 'light') {{
              document.documentElement.setAttribute('data-theme', theme);
            }} else {{
              document.documentElement.removeAttribute('data-theme');
            }}
          }}

          apply(getCookie(COOKIE_NAME));

          try {{
            const ch = new BroadcastChannel(CHANNEL_NAME);
            ch.addEventListener('message', (event) => {{
              const data = event.data;
              apply(data && data.theme);
            }});
            window.__dx_theme_channel = ch;
          }} catch (_) {{}}
        }})();
        "#,
        cookie_name = COOKIE_NAME,
        channel_name = CHANNEL_NAME
    ));
}

pub fn set_theme(dark_mode: bool) {
    let theme = if dark_mode { "dark" } else { "light" };

    _ = document::eval(&format!(
        r#"
        (function () {{
          const COOKIE_NAME = '{cookie_name}';
          const CHANNEL_NAME = '{channel_name}';

          function getCookie(name) {{
            const prefix = name + '=';
            const parts = document.cookie.split(';');
            for (let p of parts) {{
              p = p.trim();
              if (p.startsWith(prefix)) return decodeURIComponent(p.slice(prefix.length));
            }}
            return null;
          }}

          document.documentElement.setAttribute('data-theme', '{theme}');
          if (getCookie(COOKIE_NAME) === '{theme}') return;

          document.cookie = '{cookie_name}={theme}; path=/; max-age=31536000; samesite=lax';

          try {{
            const ch = window.__dx_theme_channel;
            if (ch && typeof ch.postMessage === 'function') {{
              ch.postMessage({{ theme: '{theme}' }});
            }} else {{
              const tmp = new BroadcastChannel(CHANNEL_NAME);
              tmp.postMessage({{ theme: '{theme}' }});
              tmp.close();
            }}
          }} catch (_) {{}}
        }})();
        "#,
        cookie_name = COOKIE_NAME,
        channel_name = CHANNEL_NAME
    ));
}

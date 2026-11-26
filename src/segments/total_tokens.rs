use crate::config::{IconSet, TotalTokensConfig};
use crate::icons::total_tokens_icon;
use crate::input::ContextWindowInfo;
use crate::segments::tokens::format_tokens;
use crate::theme::Palette;

pub fn render_total_tokens_segment(
    ctx: &ContextWindowInfo,
    config: &TotalTokensConfig,
    icon_set: IconSet,
    palette: &Palette,
) -> Option<String> {
    if !config.enabled {
        return None;
    }

    let total = ctx.total_turn_tokens();
    if total == 0 && config.hide_when_zero {
        return None;
    }

    let formatted = format_tokens(Some(total));
    let icon = total_tokens_icon(icon_set, config.prefix.as_deref());
    let icon_part = if icon.is_empty() {
        String::new()
    } else {
        format!("{}{}{} ", palette.label, icon, palette.reset)
    };

    let io_part = if config.show_io {
        let up_str = format_tokens(Some(ctx.total_input_tokens.unwrap_or(0)));
        let down_str = format_tokens(Some(ctx.total_output_tokens.unwrap_or(0)));
        format!(
            " ({}↑{} ↓{}{})",
            palette.dim, up_str, down_str, palette.reset
        )
    } else {
        String::new()
    };

    Some(format!(
        "{}{}{}{}{}",
        icon_part, palette.total_tokens, formatted, palette.reset, io_part
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_total_tokens_zero_hidden() {
        let ctx = ContextWindowInfo::default();
        let cfg = TotalTokensConfig::default();
        let p = Palette::for_theme("plain");
        assert!(render_total_tokens_segment(&ctx, &cfg, IconSet::Plain, &p).is_none());
    }

    #[test]
    fn test_render_total_tokens_active() {
        let ctx = ContextWindowInfo {
            total_input_tokens: Some(47_427),
            total_output_tokens: Some(4_713),
            ..Default::default()
        };
        let cfg = TotalTokensConfig::default();
        let p = Palette::for_theme("plain");

        let rendered = render_total_tokens_segment(&ctx, &cfg, IconSet::Plain, &p).unwrap();
        assert_eq!(rendered, "Total: 52k (↑47k ↓4.7k)");
    }

    #[test]
    fn test_render_total_tokens_show_io_false() {
        let ctx = ContextWindowInfo {
            total_input_tokens: Some(47_427),
            total_output_tokens: Some(4_713),
            ..Default::default()
        };
        let cfg = TotalTokensConfig {
            show_io: false,
            ..Default::default()
        };
        let p = Palette::for_theme("plain");

        let rendered = render_total_tokens_segment(&ctx, &cfg, IconSet::Plain, &p).unwrap();
        assert_eq!(rendered, "Total: 52k");
    }

    #[test]
    fn test_render_total_tokens_partial_io() {
        let p = Palette::for_theme("plain");
        let cfg = TotalTokensConfig::default();

        let input_only = ContextWindowInfo {
            total_input_tokens: Some(47_427),
            total_output_tokens: None,
            ..Default::default()
        };
        assert_eq!(
            render_total_tokens_segment(&input_only, &cfg, IconSet::Plain, &p).unwrap(),
            "Total: 47k (↑47k ↓0)"
        );

        let output_only = ContextWindowInfo {
            total_input_tokens: None,
            total_output_tokens: Some(4_713),
            ..Default::default()
        };
        assert_eq!(
            render_total_tokens_segment(&output_only, &cfg, IconSet::Plain, &p).unwrap(),
            "Total: 4.7k (↑0 ↓4.7k)"
        );
    }

    #[test]
    fn test_render_total_tokens_zero_shown_when_configured() {
        let ctx = ContextWindowInfo::default();
        let cfg = TotalTokensConfig {
            hide_when_zero: false,
            ..Default::default()
        };
        let p = Palette::for_theme("plain");

        let rendered = render_total_tokens_segment(&ctx, &cfg, IconSet::Plain, &p).unwrap();
        assert_eq!(rendered, "Total: 0 (↑0 ↓0)");

        let cfg_no_io = TotalTokensConfig {
            hide_when_zero: false,
            show_io: false,
            ..Default::default()
        };
        let rendered_no_io =
            render_total_tokens_segment(&ctx, &cfg_no_io, IconSet::Plain, &p).unwrap();
        assert_eq!(rendered_no_io, "Total: 0");
    }

    #[test]
    fn test_render_total_tokens_disabled() {
        let ctx = ContextWindowInfo {
            total_input_tokens: Some(47_427),
            total_output_tokens: Some(4_713),
            ..Default::default()
        };
        let cfg = TotalTokensConfig {
            enabled: false,
            ..Default::default()
        };
        let p = Palette::for_theme("plain");

        assert!(render_total_tokens_segment(&ctx, &cfg, IconSet::Plain, &p).is_none());
    }

    #[test]
    fn test_render_total_tokens_colored_theme_ansi_wrapping() {
        let ctx = ContextWindowInfo {
            total_input_tokens: Some(47_427),
            total_output_tokens: Some(4_713),
            ..Default::default()
        };
        let cfg = TotalTokensConfig::default();
        let p = Palette::for_theme("colorblind");

        let rendered = render_total_tokens_segment(&ctx, &cfg, IconSet::Plain, &p).unwrap();
        assert_eq!(
            rendered,
            format!(
                "{}Total:{} {}52k{} ({}↑47k ↓4.7k{})",
                p.label, p.reset, p.total_tokens, p.reset, p.dim, p.reset
            )
        );
    }
}

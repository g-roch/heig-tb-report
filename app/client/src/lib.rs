use std::fmt::Debug;
use std::fmt::Display;

use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::MultiSelect;
use dialoguer::Sort;

pub fn voting_choose<T>(items: &[T]) -> Result<Vec<&T>, Box<dyn std::error::Error>>
where
    T: Display,
    T: Debug,
    T: PartialEq,
{
    let term = Term::stderr();
    let theme = ColorfulTheme::default();
    loop {
        let selected_id = match MultiSelect::with_theme(&theme)
            .with_prompt("Selectionez les options tolérable pour vous.")
            .items(&items)
            .report(true)
            .interact_on_opt(&term)?
        {
            Some(v) => v,
            None => continue,
        };

        let selected: Vec<&T> = items
            .iter()
            .enumerate()
            .filter_map(|(id, opt)| selected_id.contains(&id).then(|| opt))
            .collect();

        let ordered_id = match Sort::with_theme(&theme)
            .with_prompt("Trier les options de la préférable à la moins accépatable.")
            .items(&selected)
            .report(true)
            .interact_on_opt(&term)?
        {
            Some(v) => v,
            None => continue,
        };

        let ordered: Vec<&T> = ordered_id.iter().map(|&id| selected[id]).collect();

        eprintln!(
            "{} {} {} {}",
            &theme.prompt_suffix,
            &theme.prompt_style.apply_to("Votre selection"),
            &theme.success_suffix,
            ordered
                .iter()
                .map(|e| format!(
                    "{} {}",
                    theme.success_prefix,
                    theme.values_style.apply_to(e)
                ))
                //.map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
        eprintln!(
            "{} {} {} {}",
            &theme.prompt_suffix,
            &theme.prompt_style.apply_to("Vous avez refusé"),
            &theme.success_suffix,
            items
                .iter()
                .filter(|e| !ordered.contains(e))
                .map(|e| format!(
                    "{} {}",
                    theme.error_prefix,
                    theme.inactive_item_style.apply_to(e)
                ))
                //.map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );

        if Confirm::with_theme(&theme)
            .with_prompt("Confirmez-vous la selection ?")
            .default(true)
            .interact_on_opt(&term)?
            .unwrap_or(false)
        {
            return Ok(ordered);
        }
    }
}

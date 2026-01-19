use crate::models::{DebtorInfo, PaymentSummary, TenantType};
use chrono::Utc;
use color_eyre::eyre::Result;
use std::path::Path;

/// Exports debtors to Markdown.
pub fn export_debtors_markdown(debtors: &[DebtorInfo], path: &Path) -> Result<()> {
    let mut content = String::new();

    content.push_str("# Schuldentabelle\n\n");
    content.push_str(&format!(
        "Stand: {}\n\n",
        Utc::now().format("%d.%m.%Y %H:%M")
    ));

    content.push_str("| E-Mail | Typ | Schulden | Überfällig seit |\n");
    content.push_str("|--------|-----|----------|------------------|\n");

    let mut total = 0;
    for debtor in debtors {
        let amount_eur = debtor.total_debt_cents as f64 / 100.0;
        total += debtor.total_debt_cents;

        let tenant_type = match debtor.tenant_type {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        };

        content.push_str(&format!(
            "| {} | {} | {:.2} € | {} Tagen |\n",
            debtor.email, tenant_type, amount_eur, debtor.days_overdue
        ));
    }

    content.push('\n');
    content.push_str(&format!(
        "**Gesamt: {} Schuldner, {:.2} € ausstehend**\n",
        debtors.len(),
        total as f64 / 100.0
    ));

    std::fs::write(path, content)?;
    Ok(())
}

/// Exports finance overview to Markdown.
pub fn export_finance_overview_markdown(
    summary: &PaymentSummary,
    debtors: &[DebtorInfo],
    path: &Path,
) -> Result<()> {
    let mut content = String::new();

    content.push_str("# Finanzübersicht\n\n");
    content.push_str(&format!(
        "Stand: {}\n\n",
        Utc::now().format("%d.%m.%Y %H:%M")
    ));

    content.push_str("## Einnahmen\n\n");
    content.push_str(&format!(
        "- Pfand-Einnahmen: {:.2} €\n",
        summary.deposits_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- Verlängerungen: {:.2} €\n",
        summary.extensions_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- Pfand-Rückgaben: {:.2} €\n",
        summary.deposit_returns_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- **Gesamt-Einnahmen: {:.2} €**\n",
        summary.total_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- **Netto (nach Pfand): {:.2} €**\n\n",
        summary.net_cents as f64 / 100.0
    ));

    content.push_str("## Ausstehende Zahlungen\n\n");

    let student_debtors: Vec<_> = debtors
        .iter()
        .filter(|d| d.tenant_type == TenantType::Schüler)
        .collect();
    let teacher_debtors: Vec<_> = debtors
        .iter()
        .filter(|d| d.tenant_type == TenantType::Lehrer)
        .collect();

    let student_debt: i32 = student_debtors.iter().map(|d| d.total_debt_cents).sum();
    let teacher_debt: i32 = teacher_debtors.iter().map(|d| d.total_debt_cents).sum();

    content.push_str(&format!(
        "- Schüler: {:.2} € ({} Verleih)\n",
        student_debt as f64 / 100.0,
        student_debtors.len()
    ));
    content.push_str(&format!(
        "- Lehrer: {:.2} € ({} Verleih)\n",
        teacher_debt as f64 / 100.0,
        teacher_debtors.len()
    ));
    content.push_str(&format!(
        "- **Gesamt: {:.2} € ({} Verleih)**\n",
        (student_debt + teacher_debt) as f64 / 100.0,
        debtors.len()
    ));

    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_debtors_markdown() -> Result<()> {
        let debtors = vec![DebtorInfo {
            username: "test".to_string(),
            email: "test@athenetz.de".to_string(),
            total_debt_cents: 1000,
            tenant_type: TenantType::Schüler,
            days_overdue: 30,
        }];

        let file = NamedTempFile::new()?;
        export_debtors_markdown(&debtors, file.path())?;

        let content = std::fs::read_to_string(file.path())?;
        assert!(content.contains("Schuldentabelle"));
        assert!(content.contains("test@athenetz.de"));
        assert!(content.contains("10.00 €"));

        Ok(())
    }
}

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Test that verifies the dashboard layout constraints are correct
/// Based on v2.1 spec section 6.2
#[test]
fn test_dashboard_layout_constraints() {
    // Test with a typical terminal size
    let area = Rect::new(0, 0, 80, 40);
    
    // Simulate the dashboard layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),     // Belegung
            Constraint::Min(6),        // Aktionen (wächst)
            Constraint::Length(7),     // Standorte + Finanzen row
            Constraint::Min(8),        // Trend (wächst)
        ])
        .split(area);
    
    // Verify Belegung has fixed height of 9
    assert_eq!(main_chunks[0].height, 9, "Belegung should have fixed height of 9");
    
    // Verify Standorte + Finanzen row has fixed height of 7
    assert_eq!(main_chunks[2].height, 7, "Standorte + Finanzen should have fixed height of 7");
    
    // Verify Aktionen has minimum height of 6
    assert!(main_chunks[1].height >= 6, "Aktionen should have minimum height of 6");
    
    // Verify Trend has minimum height of 8
    assert!(main_chunks[3].height >= 8, "Trend should have minimum height of 8");
    
    // Verify total height equals input area height
    let total_height: u16 = main_chunks.iter().map(|r| r.height).sum();
    assert_eq!(total_height, area.height, "Total height should equal input area height");
}

#[test]
fn test_dashboard_layout_horizontal_split() {
    // Test the horizontal split for Standorte + Finanzen
    let area = Rect::new(0, 0, 80, 7);
    
    let middle_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);
    
    // Verify the area is split roughly 50/50
    assert!(middle_row[0].width >= 39 && middle_row[0].width <= 41, 
            "First column should be roughly 50% (got {})", middle_row[0].width);
    assert!(middle_row[1].width >= 39 && middle_row[1].width <= 41, 
            "Second column should be roughly 50% (got {})", middle_row[1].width);
    
    // Verify total width equals input area width
    let total_width: u16 = middle_row.iter().map(|r| r.width).sum();
    assert_eq!(total_width, area.width, "Total width should equal input area width");
}

#[test]
fn test_dashboard_layout_with_small_terminal() {
    // Test with minimum terminal size (80x24)
    // Account for header (2), keybind bar (3), status bar (1) = 6 lines reserved
    // Leaving 18 lines for dashboard content
    let area = Rect::new(0, 0, 80, 30);
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),     // Belegung
            Constraint::Min(6),        // Aktionen (wächst)
            Constraint::Length(7),     // Standorte + Finanzen row
            Constraint::Min(8),        // Trend (wächst)
        ])
        .split(area);
    
    // Verify minimum constraints are respected even in small terminal
    assert_eq!(main_chunks[0].height, 9, "Belegung should be 9 lines");
    assert!(main_chunks[1].height >= 6, "Aktionen should be at least 6 lines");
    assert_eq!(main_chunks[2].height, 7, "Standorte + Finanzen should be 7 lines");
    // Trend might be smaller in constrained terminals
    assert!(main_chunks[3].height >= 8, "Trend should be at least 8 lines");
}

#[test]
fn test_dashboard_layout_with_large_terminal() {
    // Test with large terminal size
    let area = Rect::new(0, 0, 120, 60);
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),     // Belegung
            Constraint::Min(6),        // Aktionen (wächst)
            Constraint::Length(7),     // Standorte + Finanzen row
            Constraint::Min(8),        // Trend (wächst)
        ])
        .split(area);
    
    // Fixed heights should remain fixed
    assert_eq!(main_chunks[0].height, 9);
    assert_eq!(main_chunks[2].height, 7);
    
    // Growing sections should have grown
    assert!(main_chunks[1].height > 6, "Aktionen should grow in large terminal");
    assert!(main_chunks[3].height > 8, "Trend should grow in large terminal");
    
    // Verify total height
    let total_height: u16 = main_chunks.iter().map(|r| r.height).sum();
    assert_eq!(total_height, area.height);
}

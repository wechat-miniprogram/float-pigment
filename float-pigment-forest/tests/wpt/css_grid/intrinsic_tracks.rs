// WPT-style tests for intrinsic track sizing (min-content / max-content)
// Reference: CSS Grid Layout Module Level 1
//
// §7.2 Track Sizing Functions:
//   https://www.w3.org/TR/css-grid-1/#track-sizing
// §11.5 Resolve Intrinsic Track Sizes:
//   https://www.w3.org/TR/css-grid-1/#algo-content
// §11.6 Maximize Tracks:
//   https://www.w3.org/TR/css-grid-1/#algo-grow-tracks
// §11.8 Stretch auto Tracks:
//   https://www.w3.org/TR/css-grid-1/#algo-stretch
//
// These tests cover:
// - min-content and max-content track sizing functions
// - Interaction of intrinsic tracks with §11.6 Maximize (free space distribution)
// - Interaction of intrinsic tracks with §11.8 Stretch (only auto tracks stretch)
// - max-content contribution uses unconstrained layout (§11.5 Step 4)

use crate::*;

// ═══════════════════════════════════════════════════════════════════════════
// min-content columns (§7.2 + §11.5)
// ═══════════════════════════════════════════════════════════════════════════

// Case: Single min-content column with fixed-width item
// Spec points:
//   - min-content track sizes to the item's min-content contribution
//   - For a fixed-width item, min-content = item width
// In this test:
//   - Container: width=300px, columns: min-content
//   - Item: width=100px → min-content = 100px
//   - growth_limit = min-content = 100px (§11.5 Step 4)
//   - §11.6 Maximize cannot grow beyond growth_limit
#[test]
fn min_content_column_fixed_item() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0"></div>
        </div>
    "#,
        true
    )
}

// Case: min-content column with auto-width item containing nested content
// Spec points:
//   - min-content = smallest size that doesn't overflow
//   - The track's base_size and growth_limit are both set to min-content
// In this test:
//   - Container: width=300px, columns: min-content
//   - Item: auto width with 80px child → min-content = 80px
//   - growth_limit = 80px, so Maximize freezes at 80px
#[test]
fn min_content_column_nested_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: min-content column next to fixed column
// Spec points:
//   - min-content track gets its min-content contribution
//   - Fixed track gets its specified size
//   - Remaining space is NOT given to min-content track (frozen at growth_limit)
// In this test:
//   - Container: width=400px, columns: min-content 100px
//   - min-content column: 80px (child width), growth_limit = 80px
//   - Fixed column: 100px
//   - Free space: 400 - 80 - 100 = 220px
//   - §11.6: min-content track has finite growth_limit (80px), frozen there
#[test]
fn min_content_column_with_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: min-content 100px;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="100" expect_left="80"></div>
        </div>
    "#,
        true
    )
}

// Case: min-content column next to auto column
// Spec points:
//   - min-content track: growth_limit = min-content (finite)
//   - auto track: growth_limit = infinity (None)
//   - §11.6 Maximize: free space goes to auto (infinite growth_limit)
//   - §11.8 Stretch: remaining free space stretches auto tracks only
// In this test:
//   - Container: width=300px, columns: min-content auto
//   - min-content column: 80px (child width)
//   - auto column: gets remaining 220px (Maximize + Stretch)
#[test]
fn min_content_column_with_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content auto;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="220" expect_left="80"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple min-content columns
// Spec points:
//   - Each min-content track sizes independently to its items
//   - §11.6: All frozen at their respective growth_limits
// In this test:
//   - Container: width=400px, columns: min-content min-content
//   - Column 1: 100px, Column 2: 60px
//   - Both frozen, remaining 240px is not distributed
#[test]
fn multiple_min_content_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: min-content min-content;">
          <div expect_width="100">
            <div style="width: 100px; height: 50px;"></div>
          </div>
          <div expect_width="60" expect_left="100">
            <div style="width: 60px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// max-content columns (§7.2 + §11.5)
// ═══════════════════════════════════════════════════════════════════════════

// Case: Single max-content column with fixed-width item
// Spec points:
//   - max-content track sizes to the item's max-content contribution
//   - For a fixed-width item, max-content = item width
// In this test:
//   - Container: width=300px, columns: max-content
//   - Item: width=100px → max-content = 100px
//   - growth_limit = max-content = 100px (§11.5 Step 4)
#[test]
fn max_content_column_fixed_item() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0"></div>
        </div>
    "#,
        true
    )
}

// Case: max-content column with nested content
// Spec points:
//   - max-content = size with infinite available space (unconstrained)
//   - growth_limit = max-content contribution
// In this test:
//   - Container: width=300px, columns: max-content
//   - Item: auto width with 120px child → max-content = 120px
#[test]
fn max_content_column_nested_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content;">
          <div expect_width="120">
            <div style="width: 120px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: max-content column next to fixed column
// Spec points:
//   - max-content track: growth_limit = max-content (finite)
//   - §11.6: max-content track participates in Maximize but is frozen at growth_limit
// In this test:
//   - Container: width=400px, columns: max-content 100px
//   - max-content: 120px, growth_limit = 120px
//   - Remaining: 400 - 120 - 100 = 180px (not distributed to max-content)
#[test]
fn max_content_column_with_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: max-content 100px;">
          <div expect_width="120">
            <div style="width: 120px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="100" expect_left="120"></div>
        </div>
    "#,
        true
    )
}

// Case: max-content column next to auto column
// Spec points:
//   - max-content track: growth_limit = max-content (finite)
//   - auto track: growth_limit = infinity → absorbs free space
//   - §11.6: free space goes to auto track (infinite growth_limit)
//   - §11.8: remaining stretches auto tracks, not max-content
// In this test:
//   - Container: width=300px, columns: max-content auto
//   - max-content: 80px, auto gets 220px
#[test]
fn max_content_column_with_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content auto;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="220" expect_left="80"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple max-content columns
// Spec points:
//   - Each max-content column sizes to its items' max-content
//   - All frozen at growth_limit in §11.6
// In this test:
//   - Container: width=400px, columns: max-content max-content
//   - Column 1: 100px, Column 2: 60px
//   - Both frozen, 240px remaining is not distributed
#[test]
fn multiple_max_content_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: max-content max-content;">
          <div expect_width="100">
            <div style="width: 100px; height: 50px;"></div>
          </div>
          <div expect_width="60" expect_left="100">
            <div style="width: 60px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// min-content vs max-content difference
// ═══════════════════════════════════════════════════════════════════════════

// Case: min-content and max-content columns side by side
// Spec points:
//   - min-content: base_size = min-content, growth_limit = min-content
//   - max-content: base_size = min-content, growth_limit = max-content
//   - Both participate in §11.6 Maximize but are frozen at growth_limit
//   - Neither participates in §11.8 Stretch (only auto tracks stretch)
// In this test:
//   - Container: width=400px, columns: min-content max-content
//   - Both items: fixed 100px → min-content = max-content = 100px
//   - Remaining 200px not distributed (no auto tracks)
#[test]
fn min_content_and_max_content_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: min-content max-content;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Intrinsic tracks do NOT participate in §11.8 Stretch
// ═══════════════════════════════════════════════════════════════════════════

// Case: min-content track NOT stretched (§11.8)
// Spec points:
//   - §11.8: Only auto tracks are stretched to fill remaining space
//   - min-content tracks should NOT stretch even when justify-content: stretch
// In this test:
//   - Container: width=300px, columns: min-content
//   - Item: 80px → min-content column stays 80px
//   - Free space 220px is NOT distributed to min-content track
#[test]
fn min_content_column_not_stretched() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: max-content track NOT stretched (§11.8)
// Spec points:
//   - §11.8: Only auto tracks are stretched
//   - max-content tracks should NOT stretch
// In this test:
//   - Container: width=300px, columns: max-content
//   - Item: 80px → max-content column stays 80px
#[test]
fn max_content_column_not_stretched() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: auto track stretched but min-content track not (§11.8)
// Spec points:
//   - §11.8: auto tracks are stretched, intrinsic tracks are not
//   - Remaining free space after Maximize is distributed only to auto tracks
// In this test:
//   - Container: width=300px, columns: min-content auto
//   - min-content: 60px (frozen), auto: gets remaining 240px
//   - auto gets stretched, min-content does not
#[test]
fn stretch_auto_not_min_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content auto;">
          <div expect_width="60">
            <div style="width: 60px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="240" expect_left="60"></div>
        </div>
    "#,
        true
    )
}

// Case: auto track stretched but max-content track not (§11.8)
// Spec points:
//   - Same as above but for max-content
// In this test:
//   - Container: width=300px, columns: max-content auto
//   - max-content: 60px (frozen), auto: gets remaining 240px
#[test]
fn stretch_auto_not_max_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content auto;">
          <div expect_width="60">
            <div style="width: 60px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="240" expect_left="60"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Intrinsic tracks participate in §11.6 Maximize
// ═══════════════════════════════════════════════════════════════════════════

// Case: min-content track receives free space up to growth_limit (§11.6)
// Spec points:
//   - §11.6: Maximize distributes free space to all non-flex tracks
//   - min-content tracks have finite growth_limit, so they freeze there
// In this test:
//   - Container: width=300px, columns: min-content min-content
//   - Item 1: 80px, Item 2: 100px
//   - growth_limit = 80px and 100px respectively
//   - Free space = 300 - 80 - 100 = 120px
//   - Both tracks are already at their growth_limit, cannot grow further
#[test]
fn min_content_maximize_frozen_at_growth_limit() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content min-content;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div expect_width="100" expect_left="80">
            <div style="width: 100px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: max-content track with Maximize (§11.6)
// Spec points:
//   - max-content growth_limit = max-content contribution
//   - §11.6 distributes free space but freezes at growth_limit
// In this test:
//   - Container: width=300px, columns: max-content max-content
//   - Item 1: 80px, Item 2: 100px
//   - growth_limits = 80px, 100px → frozen, 120px undistributed
#[test]
fn max_content_maximize_frozen_at_growth_limit() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content max-content;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div expect_width="100" expect_left="80">
            <div style="width: 100px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Auto track growth_limit = infinity (§11.4 + §11.5 Step 4)
// Tests for Issue 3: auto tracks should absorb free space in §11.6
// ═══════════════════════════════════════════════════════════════════════════

// Case: Auto track absorbs all free space (growth_limit = infinity)
// Spec points:
//   - §11.4: auto max track sizing function → growth_limit = infinity
//   - §11.5 Step 4: increase(infinity, max-content) = infinity (max(∞, x) = ∞)
//   - §11.6: tracks with infinite growth_limit never freeze
// In this test:
//   - Container: width=300px, columns: 50px auto
//   - auto base_size = 0 (no content), growth_limit = infinity
//   - Free space: 300 - 50 - 0 = 250px → all goes to auto track
//   - auto column: 250px
#[test]
fn auto_track_absorbs_free_space() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px auto;">
          <div style="height: 50px;" expect_width="50" expect_left="0"></div>
          <div style="height: 50px;" expect_width="250" expect_left="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto track with content still absorbs free space beyond content
// Spec points:
//   - §11.5 Step 2: auto base_size = min-content contribution
//   - §11.5 Step 4: auto growth_limit stays infinity
//   - §11.6: auto track grows beyond content size to absorb free space
// In this test:
//   - Container: width=300px, columns: auto
//   - Item: 80px content → base_size = 80px, growth_limit = infinity
//   - §11.6: auto track grows to 300px (all free space absorbed)
#[test]
fn auto_track_grows_beyond_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto;">
          <div expect_width="300">
            <div style="width: 80px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: Auto track vs min-content track - auto absorbs, min-content frozen
// Spec points:
//   - auto: growth_limit = infinity → absorbs free space
//   - min-content: growth_limit = min-content → frozen at content size
//   - §11.6 Maximize: free space goes to auto only
// In this test:
//   - Container: width=300px, columns: auto min-content
//   - auto: 60px content, min-content: 60px content
//   - Free space: 300 - 60 - 60 = 180px
//   - §11.6: min-content frozen at 60px, auto gets 180px → total 240px
#[test]
fn auto_absorbs_min_content_frozen() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto min-content;">
          <div expect_width="240">
            <div style="width: 60px; height: 50px;"></div>
          </div>
          <div expect_width="60" expect_left="240">
            <div style="width: 60px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: Auto track vs max-content track - auto absorbs, max-content frozen
// Spec points:
//   - auto: growth_limit = infinity → absorbs free space
//   - max-content: growth_limit = max-content → frozen at content size
// In this test:
//   - Container: width=300px, columns: auto max-content
//   - auto: 60px content, max-content: 60px content
//   - Free space: 300 - 60 - 60 = 180px
//   - §11.6: max-content frozen at 60px, auto gets 180px → total 240px
#[test]
fn auto_absorbs_max_content_frozen() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto max-content;">
          <div expect_width="240">
            <div style="width: 60px; height: 50px;"></div>
          </div>
          <div expect_width="60" expect_left="240">
            <div style="width: 60px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// min-content / max-content rows
// ═══════════════════════════════════════════════════════════════════════════

// Case: min-content row with fixed-height item
// Spec points:
//   - min-content row sizes to item's min-content height
// In this test:
//   - Container: height=300px, rows: min-content
//   - Item: height=80px → min-content row = 80px
//   - growth_limit = 80px → frozen, remaining 220px not distributed
#[test]
fn min_content_row_fixed_item() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: min-content;">
          <div style="height: 80px;" expect_height="80" expect_top="0"></div>
        </div>
    "#,
        true
    )
}

// Case: max-content row with fixed-height item
// Spec points:
//   - max-content row sizes to item's max-content height
// In this test:
//   - Container: height=300px, rows: max-content
//   - Item: height=80px → max-content row = 80px
//   - growth_limit = 80px → frozen
#[test]
fn max_content_row_fixed_item() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: max-content;">
          <div style="height: 80px;" expect_height="80" expect_top="0"></div>
        </div>
    "#,
        true
    )
}

// Case: min-content row does NOT stretch (§11.8)
// Spec points:
//   - §11.8: Only auto rows stretch, not min-content
// In this test:
//   - Container: height=300px, rows: min-content auto
//   - min-content row: 50px (frozen), auto row: gets remaining 250px
//   - Second item has no height → stretched by align-self: stretch to 250px
#[test]
fn min_content_row_not_stretched() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: min-content auto;">
          <div style="height: 50px;" expect_height="50" expect_top="0"></div>
          <div expect_top="50" expect_height="250"></div>
        </div>
    "#,
        true
    )
}

// Case: max-content row does NOT stretch (§11.8)
// Spec points:
//   - §11.8: Only auto rows stretch, not max-content
// In this test:
//   - Container: height=300px, rows: max-content auto
//   - max-content row: 50px (frozen), auto row: gets remaining 250px
//   - Second item has no height → stretched by align-self: stretch to 250px
#[test]
fn max_content_row_not_stretched() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: max-content auto;">
          <div style="height: 50px;" expect_height="50" expect_top="0"></div>
          <div expect_top="50" expect_height="250"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Mixed intrinsic tracks with fr (§11.7)
// ═══════════════════════════════════════════════════════════════════════════

// Case: min-content + fr columns
// Spec points:
//   - min-content track: sized first (§11.5), growth_limit = min-content
//   - fr track: takes remaining space after min-content (§11.7)
// In this test:
//   - Container: width=300px, columns: min-content 1fr
//   - min-content: 80px, fr: 300 - 80 = 220px
#[test]
fn min_content_column_with_fr() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: min-content 1fr;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="220" expect_left="80"></div>
        </div>
    "#,
        true
    )
}

// Case: max-content + fr columns
// Spec points:
//   - max-content track: sized first (§11.5)
//   - fr track: takes remaining space (§11.7)
// In this test:
//   - Container: width=300px, columns: max-content 1fr
//   - max-content: 80px, fr: 300 - 80 = 220px
#[test]
fn max_content_column_with_fr() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: max-content 1fr;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="220" expect_left="80"></div>
        </div>
    "#,
        true
    )
}

// Case: min-content + max-content + fr columns
// Spec points:
//   - All intrinsic tracks sized first
//   - fr takes whatever is left
// In this test:
//   - Container: width=400px, columns: min-content max-content 1fr
//   - min-content: 60px, max-content: 80px
//   - fr: 400 - 60 - 80 = 260px
#[test]
fn min_content_max_content_fr_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: min-content max-content 1fr;">
          <div expect_width="60">
            <div style="width: 60px; height: 50px;"></div>
          </div>
          <div expect_width="80" expect_left="60">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="260" expect_left="140"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Intrinsic tracks with gap
// ═══════════════════════════════════════════════════════════════════════════

// Case: min-content columns with gap
// Spec points:
//   - Gap is subtracted from available space before track sizing
//   - min-content tracks still size to content
// In this test:
//   - Container: width=300px, gap=20px, columns: min-content min-content
//   - Column 1: 80px, Column 2: 60px
//   - Total: 80 + 20 + 60 = 160px (< 300px, 140px free space undistributed)
#[test]
fn min_content_columns_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; column-gap: 20px; grid-template-columns: min-content min-content;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div expect_width="60" expect_left="100">
            <div style="width: 60px; height: 50px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: max-content column with auto and gap
// Spec points:
//   - Gap subtracted from available space
//   - max-content frozen at growth_limit, auto absorbs remaining
// In this test:
//   - Container: width=300px, gap=20px, columns: max-content auto
//   - max-content: 80px, gap: 20px
//   - auto: 300 - 80 - 20 = 200px
#[test]
fn max_content_auto_columns_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; column-gap: 20px; grid-template-columns: max-content auto;">
          <div expect_width="80">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_width="200" expect_left="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Auto-width container with intrinsic tracks
// ═══════════════════════════════════════════════════════════════════════════

// Case: Auto-width container with min-content column
// Spec points:
//   - When container has no definite width, min-content track
//     sizes to content and container shrinks to fit
// In this test:
//   - Container: auto width (no width set), columns: min-content
//   - Item: 100px → container width = 100px
#[test]
fn auto_container_min_content_column() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: min-content;">
          <div style="width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto-width container with max-content column
// Spec points:
//   - When container has no definite width, max-content track
//     sizes to content and container shrinks to fit
// In this test:
//   - Container: auto width, columns: max-content
//   - Item: 100px → container width = 100px
#[test]
fn auto_container_max_content_column() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: max-content;">
          <div style="width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Complex mixed scenarios
// ═══════════════════════════════════════════════════════════════════════════

// Case: All track types together: fixed + min-content + max-content + auto + fr
// Spec points:
//   - Fixed: exact size
//   - min-content: sizes to content, frozen at growth_limit
//   - max-content: sizes to content, frozen at growth_limit
//   - auto: absorbs free space in §11.6, stretches in §11.8
//   - fr: takes remaining after all other tracks (§11.7)
// In this test:
//   - Container: width=500px, columns: 50px min-content max-content auto 1fr
//   - fixed: 50px
//   - min-content: 60px (child width)
//   - max-content: 80px (child width)
//   - Subtotal non-fr: 50 + 60 + 80 = 190px
//   - Remaining for auto + fr: 500 - 190 = 310px
//   - fr takes remaining after auto content: auto has no content → 0px base
//   - fr: all remaining goes to fr since auto starts at 0 base
#[test]
fn all_track_types_mixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 500px; grid-template-columns: 50px min-content max-content auto 1fr;">
          <div style="height: 50px;" expect_width="50" expect_left="0"></div>
          <div expect_width="60" expect_left="50">
            <div style="width: 60px; height: 50px;"></div>
          </div>
          <div expect_width="80" expect_left="110">
            <div style="width: 80px; height: 50px;"></div>
          </div>
          <div style="height: 50px;" expect_left="190"></div>
          <div style="height: 50px;" expect_left="190"></div>
        </div>
    "#,
        true
    )
}

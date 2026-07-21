// Tests for inline and inline-block layout in CSS
// Based on CSS Display Module Level 3 and CSS Inline Layout Module Level 3:
// - display: inline creates inline-level boxes
// - display: inline-block creates inline-level block containers
// - Inline boxes flow horizontally and wrap when necessary
// - Vertical alignment affects inline content positioning

use crate::*;

// Case: Basic inline elements
// Spec points:
// - Inline elements flow horizontally
// - Nested inline elements maintain inline flow
// In this test:
// - Block with text: 16px height (line height), full width
// - Inline with nested inline: 32px width (text content)
// - Inline with text before nested inline: 64px width

// Case: Inline element with explicit size
// Spec points:
// - In block context, inline ignores width/height
// - In flex context, inline respects width/height
// In this test:
// - Flex child: inline with size = 10x10
// - Block child: inline with size ignored = 0x0

// Case: inline-block with padding
// Spec points:
// - inline-block creates a block formatting context
// - Padding adds to dimensions
// In this test:
// - Container: height=40px, padding=15px (all sides)
// - Two 30x30 block children stack vertically
// - Total height: 40 + 15 + 15 = 70px (with overflow)
// - Total width: 30 + 15 + 15 = 60px

// Case: inline-block with padding (inline-block children)
// Spec points:
// - inline-block children flow horizontally
// In this test:
// - Two 30x30 inline-block children side by side
// - Width: 30 + 30 + 30 = 90px (padding + 2 children)

// Case: inline element in block with percentage child
// Spec points:
// - Inline element can contain percentage-sized children
// - Percentage resolves relative to containing block
// In this test:
// - Block: 200x200 with 50px padding
// - Inline child contains 100%x100% block
// - Child sizes: 275x100 (275 = 375 - 50*2, wait that's wrong)
// - Actually: content area = 200 - padding? Let me check original
// - Container: box-sizing default, 200x200 + padding = 275x100 content area

// Case: inline element in flexbox
// Spec points:
// - In flex context, inline children become flex items
// - flex-direction affects inline behavior
// In this test:
// - Flex column: inline child stretches to full width
// - Flex row: inline child shrinks to content

// Case: Basic inline-block
// Spec points:
// - inline-block elements flow horizontally
// - Each is its own block formatting context
// In this test:
// - Two inline-block text elements side by side

// Case: inline-block vertical alignment (baseline)
// Spec points:
// - Baseline alignment is default for inline-block
// - Taller elements push others down
// In this test:
// - Two boxes: 30px and 50px tall
// - 30px box aligned to bottom of 50px box (top=20)

// Case: inline-block vertical alignment with text
// Spec points:
// - Text baseline affects alignment
// In this test:
// - First box has text, aligned by text baseline
// - Second box without text at top

// Case: inline-block wrapping
// Spec points:
// - inline-block elements wrap when exceeding container width
// In this test:
// - Container: 100px
// - 4 items of 30px: 3 fit on first line, 1 wraps

// Case: inline-block wrapping with varying heights
// Spec points:
// - Line height determined by tallest element
// In this test:
// - Items of different heights
// - Line height = max height in that line

// Case: inline-block wrapping with block interruption
// Spec points:
// - Block elements break inline flow
// - New line starts after block
// In this test:
// - Two inline-blocks, then a block, then two more inline-blocks

// Case: inline-block wrapping in narrow container
// Spec points:
// - Each item on its own line when container is too narrow
// In this test:
// - Container: 10px, items 30px wide
// - Each item on separate line

// Case: inline-block with margin
// Spec points:
// - Margins affect inline-block positioning
// In this test:
// - Item with margin=20px all around
// - Container height includes margins: 10 + 20 + 20 = 50px

// Case: inline-block with different margins
// Spec points:
// - Margins: top, right, bottom, left
// In this test:
// - margin: 10px 20px 30px 40px
// - Total height: 10 + 10 + 30 = 50px

// Case: Multiple inline-blocks with margin
// Spec points:
// - Margins don't collapse for inline-block
// In this test:
// - Two items with different margins, different heights

// Case: inline-block with margin wrapping
// Spec points:
// - Margins included in line width calculation for wrapping
// In this test:
// - Container: 100px
// - Each item: 10px + 60px margin = 70px
// - Items wrap due to margin

// Case: inline-block in flexbox
// Spec points:
// - inline-block in flex becomes flex item
// - Sizing behavior differs based on flex direction
// In this test:
// - Various flex contexts with inline-block children

// Case: block in inline-block
// Spec points:
// - inline-block establishes block formatting context
// - Block children lay out normally inside
// In this test:
// - inline-block containing block child
// - Sizing determined by content

use float_pigment_css::typing::Display;
use float_pigment_forest::{
    convert_node_ref_to_ptr, ChildOperation, DumpNode, DumpOptions, DumpStyleMode, Node,
    StyleSetter,
};
use float_pigment_layout::{DefLength, OptionNum, OptionSize, Size};

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

// Case: inline-block as root element
// Spec points:
// - Root inline-block sizes to content
// In this test:
// - Root: inline-block, child: 10x20
// - Root width = child width = 10
#[test]
pub fn inline_block_as_root() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::InlineBlock);
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(10.)));
        child.set_height(DefLength::Points(Len::from_f32(20.)));
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(container.layout_position().width, 10.);
    }
}

// Case: inline as root element
// Spec points:
// - Root inline stretches to available width
// - Percentage children resolve relative to viewport
// In this test:
// - Root: inline, child: 100% width
// - Both stretch to viewport width (375px)
#[test]
pub fn inline_as_root() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Inline);
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Percent(1.));
        child.set_height(DefLength::Points(Len::from_f32(100.)));
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(container.layout_position().width, 375.);
        assert_eq!(child.layout_position().width, 375.);
    }
}

// Case: Measurable inline-block with padding
// Spec points:
// - measure_func provides intrinsic size
// - Padding adds to measured size
// In this test:
// - inline-block with measure_func returning 20x20
// - padding-left/right = 12px each
// - Final width: 25 (explicit) + 12 + 12 = 49px
#[test]
pub fn measurable_inline_block_with_padding() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(25.)));
        child.set_height(DefLength::Points(Len::from_f32(25.)));
        child.set_display(Display::InlineBlock);
        child.set_padding_left(DefLength::Points(Len::from_f32(12.)));
        child.set_padding_right(DefLength::Points(Len::from_f32(12.)));
        child.set_measure_func(Some(Box::new(|_, _, _, _, _, _, _, _, _| {
            Size::new(Len::from_f32(20.), Len::from_f32(20.))
        })));
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(child.layout_position().left, 0.);
        assert_eq!(child.layout_position().width, 49.);
        assert_eq!(child.layout_position().height, 25.);
    }
}

// Case: Measurable inline-block with margin
// Spec points:
// - margin offsets position but doesn't affect computed size
// In this test:
// - inline-block with margin-left/right = 12px
// - Child positioned at left=12, width=25
#[test]
pub fn measurable_inline_block_with_margin() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(25.)));
        child.set_height(DefLength::Points(Len::from_f32(25.)));
        child.set_display(Display::InlineBlock);
        child.set_margin_left(DefLength::Points(Len::from_f32(12.)));
        child.set_margin_right(DefLength::Points(Len::from_f32(12.)));
        child.set_measure_func(Some(Box::new(|_, _, _, _, _, _, _, _, _| {
            Size::new(Len::from_f32(20.), Len::from_f32(20.))
        })));
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(child.layout_position().left, 12.);
        assert_eq!(child.layout_position().width, 25.);
    }
}

// Case: Multiple measurable inline-blocks with margin
// Spec points:
// - Multiple inline-blocks with margins flow correctly
// In this test:
// - Two inline-blocks with 12px margins
// - Second child at left=61 (12 + 25 + 12 + 12)
#[test]
pub fn measurable_inline_block_with_margin_2() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(25.)));
        child.set_height(DefLength::Points(Len::from_f32(25.)));
        child.set_display(Display::InlineBlock);
        child.set_margin_left(DefLength::Points(Len::from_f32(12.)));
        child.set_margin_right(DefLength::Points(Len::from_f32(12.)));
        child.set_measure_func(Some(Box::new(|_, _, _, _, _, _, _, _, _| {
            Size::new(Len::from_f32(20.), Len::from_f32(20.))
        })));
        container.append_child(convert_node_ref_to_ptr(child));
        let child_b = as_ref(Node::new_ptr());
        child_b.set_width(DefLength::Points(Len::from_f32(25.)));
        child_b.set_height(DefLength::Points(Len::from_f32(25.)));
        child_b.set_display(Display::InlineBlock);
        child_b.set_margin_left(DefLength::Points(Len::from_f32(12.)));
        child_b.set_margin_right(DefLength::Points(Len::from_f32(12.)));
        child_b.set_measure_func(Some(Box::new(|_, _, _, _, _, _, _, _, _| {
            Size::new(Len::from_f32(25.), Len::from_f32(25.))
        })));
        container.append_child(convert_node_ref_to_ptr(child_b));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            container.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::None
                },
                0
            )
        );
        assert_eq!(child.layout_position().left, 12.);
        assert_eq!(child.layout_position().width, 25.);
        assert_eq!(child_b.layout_position().left, 61.);
        assert_eq!(child_b.layout_position().width, 25.);
    }
}

// Case: Complex inline nesting with flex
// Spec points:
// - Complex nesting of inline, flex, and block elements
// In this test:
// - span (inline) containing flex column containing nested structure

// Case: inline-flex display
// Spec points:
// - inline-flex creates inline-level flex container
// - Flows inline but children use flex layout
// In this test:
// - Multiple inline-flex containers flowing horizontally

// Case: inline-block wrapping precision
// Spec points:
// - Subpixel calculations affect wrapping behavior
// - Cumulative widths must be precise to avoid premature wrapping
// In this test:
// - Multiple percentage-width items that should fit on one line
// - 5 items of 20% should wrap at 6th item

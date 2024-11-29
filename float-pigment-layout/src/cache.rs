use crate::*;
use lru::LruCache;

const LAYOUT_LRU_CACHE_LEN_PER_NODE: usize = 8;

pub type CacheKeyRequestSize<L> = OptionSize<<L as LengthNum>::Hashable>;
pub type CacheKeyMaxContent<L> = OptionSize<<L as LengthNum>::Hashable>;
pub type CacheKeyParentSize<L> = OptionSize<<L as LengthNum>::Hashable>;

#[cfg_attr(debug_assertions, derive(Debug))]
#[allow(clippy::type_complexity)]
pub(crate) struct LayoutComputeCache<L: LengthNum> {
    touched: bool,
    parent_size_affected: bool,
    size_cache: LruCache<
        (
            CacheKeyRequestSize<L>,
            CacheKeyMaxContent<L>,
            CacheKeyParentSize<L>,
        ),
        ComputeResult<L>,
    >,
    position_cache: Option<(
        (
            CacheKeyRequestSize<L>,
            CacheKeyMaxContent<L>,
            CacheKeyParentSize<L>,
        ),
        ComputeResult<L>,
    )>,
}

impl<L: LengthNum> LayoutComputeCache<L> {
    pub(crate) fn new() -> Self {
        Self {
            touched: false,
            parent_size_affected: false,
            size_cache: LruCache::new(0),
            position_cache: None,
        }
    }

    pub(crate) fn calc_parent_size_affected(node: &impl LayoutTreeNode) -> bool {
        let style = node.style();
        if let DefLength::Percent(_) = style.min_width() {
            return true;
        };
        if let DefLength::Percent(_) = style.min_height() {
            return true;
        };
        if let DefLength::Percent(_) = style.max_width() {
            return true;
        };
        if let DefLength::Percent(_) = style.max_height() {
            return true;
        };
        if let DefLength::Percent(_) = style.margin_left() {
            return true;
        };
        if let DefLength::Percent(_) = style.margin_right() {
            return true;
        };
        if let DefLength::Percent(_) = style.margin_top() {
            return true;
        };
        if let DefLength::Percent(_) = style.margin_bottom() {
            return true;
        };
        if let DefLength::Percent(_) = style.padding_left() {
            return true;
        };
        if let DefLength::Percent(_) = style.padding_right() {
            return true;
        };
        if let DefLength::Percent(_) = style.padding_top() {
            return true;
        };
        if let DefLength::Percent(_) = style.padding_bottom() {
            return true;
        };
        if let DefLength::Percent(_) = style.border_left() {
            return true;
        };
        if let DefLength::Percent(_) = style.border_right() {
            return true;
        };
        if let DefLength::Percent(_) = style.border_top() {
            return true;
        };
        if let DefLength::Percent(_) = style.border_bottom() {
            return true;
        };
        false
    }

    pub(crate) fn clear(&mut self) -> bool {
        self.size_cache.clear();
        self.clear_position_cache();
        let ret = self.touched;
        self.touched = false;
        ret
    }

    #[inline(always)]
    pub(crate) fn clear_position_cache(&mut self) {
        self.position_cache = None;
    }

    pub(crate) fn touch(&mut self, node: &impl LayoutTreeNode) {
        if !self.touched {
            self.touched = true;
            self.parent_size_affected = Self::calc_parent_size_affected(node);
        }
    }

    pub(crate) fn gen_key(
        &mut self,
        node: &impl LayoutTreeNode,
        req: &ComputeRequest<L>,
    ) -> (
        CacheKeyRequestSize<L>,
        CacheKeyMaxContent<L>,
        CacheKeyParentSize<L>,
    ) {
        self.touch(node);
        let p = if self.parent_size_affected {
            *req.parent_inner_size
        } else {
            OptionSize::new(OptionNum::none(), OptionNum::none())
        };
        let size = Size::new(req.size.width.to_hashable(), req.size.height.to_hashable());
        let max_content = Size::new(
            req.max_content.width.to_hashable(),
            req.max_content.height.to_hashable(),
        );
        let p = Size::new(p.width.to_hashable(), p.height.to_hashable());
        (size, max_content, p)
    }

    pub(crate) fn write_all_size_inner(
        &mut self,
        node: &impl LayoutTreeNode<Length = L>,
        req: &ComputeRequest<L>,
        result: ComputeResult<L>,
    ) {
        let key = self.gen_key(node, req);
        if self.size_cache.cap() == 0 {
            self.size_cache.resize(LAYOUT_LRU_CACHE_LEN_PER_NODE);
        }
        self.size_cache.put(key, result);
    }

    pub(crate) fn write_all_size(
        &mut self,
        node: &impl LayoutTreeNode<Length = L>,
        req: &ComputeRequest<L>,
        result: ComputeResult<L>,
    ) {
        if req.parent_is_block {
            // do not store cache if parent is common block
            return;
        }
        self.write_all_size_inner(node, req, result);
        if req.size.width.is_none() || req.size.height.is_none() {
            let mut req = req.clone();
            req.size = Normalized(OptionSize::new(
                req.size.width.or(OptionNum::some(result.size.width)),
                req.size.height.or(OptionNum::some(result.size.height)),
            ));
            self.write_all_size_inner(node, &req, result);
            if req.max_content.width.is_none() || req.max_content.height.is_none() {
                req.max_content = Normalized(OptionSize::new(
                    req.max_content.width.or(OptionNum::some(result.size.width)),
                    req.max_content
                        .height
                        .or(OptionNum::some(result.size.height)),
                ));
                self.write_all_size_inner(node, &req, result);
            }
        }
        if req.max_content.width.is_none() || req.max_content.height.is_none() {
            let mut req = req.clone();
            req.max_content = Normalized(OptionSize::new(
                req.max_content.width.or(OptionNum::some(result.size.width)),
                req.max_content
                    .height
                    .or(OptionNum::some(result.size.height)),
            ));
            self.write_all_size_inner(node, &req, result);
        }
    }

    pub(crate) fn write_position<T: LayoutTreeNode<Length = L>>(
        &mut self,
        node: &T,
        req: &ComputeRequest<L>,
        result: ComputeResult<L>,
    ) {
        if !req.parent_is_block {
            // do not store cache if parent is common block
            self.write_all_size(node, req, result);
        } else if self.position_cache.is_none() {
            self.parent_size_affected = Self::calc_parent_size_affected(node);
        }
        let key = self.gen_key(node, req);
        self.position_cache = Some((key, result));
    }

    pub(crate) fn read(
        &mut self,
        node: &impl LayoutTreeNode<Length = L>,
        req: &ComputeRequest<L>,
    ) -> Option<ComputeResult<L>> {
        if !self.touched {
            return None;
        }
        let key = self.gen_key(node, req);
        if req.kind != ComputeRequestKind::Position {
            self.size_cache.get(&key).cloned()
        } else {
            self.position_cache
                .as_ref()
                .and_then(|(k, res)| if *k == key { Some(*res) } else { None })
        }
    }
}

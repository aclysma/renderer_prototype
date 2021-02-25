// This code is auto-generated by the shader processor.

#[allow(unused_imports)]
use rafx_framework::RafxResult;

#[allow(unused_imports)]
use rafx_framework::{
    DescriptorSetAllocator, DescriptorSetArc, DescriptorSetInitializer, DynDescriptorSet,
    ImageViewResource, ResourceArc,
};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PerViewUboStd140 {
    pub view_proj: [[f32; 4]; 4], // +0 (size: 64)
} // 64 bytes

impl Default for PerViewUboStd140 {
    fn default() -> Self {
        PerViewUboStd140 {
            view_proj: <[[f32; 4]; 4]>::default(),
        }
    }
}

pub type PerViewUboUniform = PerViewUboStd140;

pub const PER_VIEW_DATA_DESCRIPTOR_SET_INDEX: usize = 1;
pub const PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX: usize = 0;
pub const TEX_DESCRIPTOR_SET_INDEX: usize = 0;
pub const TEX_DESCRIPTOR_BINDING_INDEX: usize = 0;
pub const SMP_DESCRIPTOR_SET_INDEX: usize = 0;
pub const SMP_DESCRIPTOR_BINDING_INDEX: usize = 1;

pub struct DescriptorSet0Args<'a> {
    pub tex: &'a ResourceArc<ImageViewResource>,
}

impl<'a> DescriptorSetInitializer<'a> for DescriptorSet0Args<'a> {
    type Output = DescriptorSet0;

    fn create_dyn_descriptor_set(
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> Self::Output {
        let mut descriptor = DescriptorSet0(descriptor_set);
        descriptor.set_args(args);
        descriptor
    }

    fn create_descriptor_set(
        descriptor_set_allocator: &mut DescriptorSetAllocator,
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> RafxResult<DescriptorSetArc> {
        let mut descriptor = Self::create_dyn_descriptor_set(descriptor_set, args);
        descriptor.0.flush(descriptor_set_allocator)?;
        Ok(descriptor.0.descriptor_set().clone())
    }
}

pub struct DescriptorSet0(pub DynDescriptorSet);

impl DescriptorSet0 {
    pub fn set_args_static(
        descriptor_set: &mut DynDescriptorSet,
        args: DescriptorSet0Args,
    ) {
        descriptor_set.set_image(TEX_DESCRIPTOR_BINDING_INDEX as u32, args.tex);
    }

    pub fn set_args(
        &mut self,
        args: DescriptorSet0Args,
    ) {
        self.set_tex(args.tex);
    }

    pub fn set_tex(
        &mut self,
        tex: &ResourceArc<ImageViewResource>,
    ) {
        self.0.set_image(TEX_DESCRIPTOR_BINDING_INDEX as u32, tex);
    }

    pub fn flush(
        &mut self,
        descriptor_set_allocator: &mut DescriptorSetAllocator,
    ) -> RafxResult<()> {
        self.0.flush(descriptor_set_allocator)
    }
}

pub struct DescriptorSet1Args<'a> {
    pub per_view_data: &'a PerViewUboUniform,
}

impl<'a> DescriptorSetInitializer<'a> for DescriptorSet1Args<'a> {
    type Output = DescriptorSet1;

    fn create_dyn_descriptor_set(
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> Self::Output {
        let mut descriptor = DescriptorSet1(descriptor_set);
        descriptor.set_args(args);
        descriptor
    }

    fn create_descriptor_set(
        descriptor_set_allocator: &mut DescriptorSetAllocator,
        descriptor_set: DynDescriptorSet,
        args: Self,
    ) -> RafxResult<DescriptorSetArc> {
        let mut descriptor = Self::create_dyn_descriptor_set(descriptor_set, args);
        descriptor.0.flush(descriptor_set_allocator)?;
        Ok(descriptor.0.descriptor_set().clone())
    }
}

pub struct DescriptorSet1(pub DynDescriptorSet);

impl DescriptorSet1 {
    pub fn set_args_static(
        descriptor_set: &mut DynDescriptorSet,
        args: DescriptorSet1Args,
    ) {
        descriptor_set.set_buffer_data(
            PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX as u32,
            args.per_view_data,
        );
    }

    pub fn set_args(
        &mut self,
        args: DescriptorSet1Args,
    ) {
        self.set_per_view_data(args.per_view_data);
    }

    pub fn set_per_view_data(
        &mut self,
        per_view_data: &PerViewUboUniform,
    ) {
        self.0
            .set_buffer_data(PER_VIEW_DATA_DESCRIPTOR_BINDING_INDEX as u32, per_view_data);
    }

    pub fn flush(
        &mut self,
        descriptor_set_allocator: &mut DescriptorSetAllocator,
    ) -> RafxResult<()> {
        self.0.flush(descriptor_set_allocator)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_struct_per_view_ubo_std140() {
        assert_eq!(std::mem::size_of::<PerViewUboStd140>(), 64);
        assert_eq!(std::mem::size_of::<[[f32; 4]; 4]>(), 64);
        assert_eq!(std::mem::align_of::<[[f32; 4]; 4]>(), 4);
        assert_eq!(memoffset::offset_of!(PerViewUboStd140, view_proj), 0);
    }
}
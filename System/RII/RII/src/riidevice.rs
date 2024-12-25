trait rii_device
{
	fn get_rendering_queue(&self) -> rii_queue;
	fn get_compute_queue(&self) -> rii_queue;
	fn get_dma_queue(&self) -> rii_queue;
	fn allocate_buffer(&self, size: usize) -> rii_buffer;
};
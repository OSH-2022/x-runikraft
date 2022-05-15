#![no_std]

use rkalloc::RKalloc;
use crate::RkBlkdev;

/**
 * Queue Structure used for both requests and responses.
 * This is private to the drivers.
 * In the API, this structure is used only for type checking.
 * Structure used to describe a queue used for both requests and responses
 */
#[cfg(feature = "xen_blkfront_grefpool")]
pub struct RkBlkdevQueue {
    /* Front_ring structure */
    //TODO struct blkif_front_ring ring,
    /* Grant ref pointing at the front ring. */
    //TODO grant_ref_t ring_ref;
    /* Event channel for the front ring. */
    //TODO evtchn_port_t evtchn;
    /* Allocator for this queue. */
    a: dyn RKalloc,
    /* The libukblkdev queue identifier */
    queue_id:u16,
    /* The flag to interrupt on the queue */
    intr_enabled:isize,
    /* Reference to the Blkfront Device */
    /* Grant refs pool. */
    //TODO struct blkfront_grefs_pool ref_pool,
}
#[cfg(not(feature = "xen_blkfront_grefpool"))]
pub struct RkBlkdevQueue {
    /* Front_ring structure */
    //TODO struct blkif_front_ring ring,
    /* Grant ref pointing at the front ring. */
    //TODO grant_ref_t ring_ref;
    /* Event channel for the front ring. */
    //TODO evtchn_port_t evtchn;
    /* Allocator for this queue. */
    a: dyn RKalloc,
    /* The libukblkdev queue identifier */
    queue_id:u16,
    /* The flag to interrupt on the queue */
    intr_enabled:isize,
    /* Reference to the Blkfront Device */
}
/**
 * Structure used to describe the Blkfront device.
 */
pub struct BlkfrontDev <'a>{
    /* Xenbus Device. */
    //TODO struct xenbus_device *xendev;
    /* Blkdev Device. */
    blkdev:RkBlkdev<'a>,
    /* Blkfront device number from Xenstore path. */
    //TODO blkif_vdev_t	handle;
    /* Value which indicates that the backend can process requests with the
     * BLKIF_OP_WRITE_BARRIER request opcode.
     */
    barrier:isize,
    /* Value which indicates that the backend can process requests with the
     * BLKIF_OP_WRITE_FLUSH_DISKCACHE request opcode.
     */
    flush:int,
    /* Number of configured queues used for requests */
    nb_queues:u16,
    /* Vector of queues used for communication with backend */
    queues:*mut RkBlkdevQueue,
    /* The blkdev identifier */
    uid:u16,
}
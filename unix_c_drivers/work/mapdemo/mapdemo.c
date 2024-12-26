/* work just kernerl v4. Did't compiled.*/


// #include "radio.h"
#include <linux/cdev.h>
#include <linux/slab.h>
#include <linux/mm.h>
#include <linux/mm_types.h>


#define DEV_NAME "mmap-radio"


MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

static dev_t radio_first;
static int radio_init;
static struct class *radio_class;
static struct cdev radio_dev;

struct mmap_info{
    char *msg;
    int counter;
};

void vm_open(struct vm_area_struct *vma)
{
    struct mmap_info *info = (struct mmap_info*)vma->vm_private_data;
    info->counter++;
}

void vm_close(struct vm_area_struct *vma)
{
    struct mmap_info *info = (struct mmap_info*)vma->vm_private_data;
    info->counter--;
}

vm_fault_t vm_fault(struct vm_fault *vmf){
    struct page *page;
    
    struct vm_area_struct *vma = vmf->vma;

    struct mmap_info *info = (struct mmap_info*) vma->vm_private_data;
    if(!info->msg) return 0;
    page - virt_to_page(info->msg);
    get_page(page);
    vmf->page = page;

    return 0;
}

struct vm_operations_struct vmops = {
    .open = vm_open,
    .close = vm_close,
    .fault = vm_fault
// is not error. What doing if mamory is not accesseble
};







int map_release(struct inode *node, struct file *file)
{
    struct mmap_info *info = file->private_data;
    free_page((unsigned long)info->msg);
    kfree(info);

    file->private_data = NULL;
    return 0;    
}

int mmap_func(struct file *file, struct vm_area_struct *vma)
{
    vma->vm_ops = &vmops;
    // vma->vm_flags |= VM_DONTEXPAND | VM_DONTDUMP; // is not actual for v6 kernel
    vm_flags_set(vma, VM_DONTEXPAND | VM_DONTDUMP);
    vma->vm_private_data = file->private_data;
    vm_open(vma);
    return 0;
}


int map_open(struct inode *node, struct file *file)
{
    struct mmap_info *info =NULL;
    char *msg = "This is map radio speaking";
    if(!(info = kmalloc(sizeof(struct mmap_info), GFP_KERNEL))) return -ENOMEM;
     

     
    info->msg = (char*)get_zeroed_page(GFP_KERNEL);
    memcpy(info->msg, msg, strlen(msg));

    file->private_data = info;
    return 0;

    
}


struct file_operations fops = 
{
    .owner = THIS_MODULE,
    .open = map_open,
    .release  = map_release,
    .mmap = mmap_func
};





int init_module(void)
{
    memset(&fops, 0, sizeof(fops));
    fops.owner = THIS_MODULE;
    fops.open = map_open;
    fops.release = map_release;
    
    radio_init = alloc_chrdev_region(&radio_first, 0, 1, DEV_NAME);
    if(radio_init < 0){
        printk(KERN_ALERT DEV_NAME " cannot alloc region\n");
        return radio_init;
    }

    if ((radio_class = class_create("chdevice")) == NULL)
    {
        unregister_chrdev_region(radio_first, 1);
        printk(KERN_ALERT DEV_NAME " cannot create class\n");
        return -ENODEV;
    }

    if((device_create(radio_class, NULL, radio_first, NULL, DEV_NAME)) == NULL)
    {
        class_destroy(radio_class);
        unregister_chrdev_region(radio_first, 1);
        printk(KERN_ALERT DEV_NAME " cannot create device\n");
        return -ENODEV;
    }


    cdev_init(&radio_dev, &fops);
    if((cdev_add(&radio_dev, radio_first, 1)) == -1)
    {
        device_destroy(radio_class, radio_first);
        class_destroy(radio_class);
        unregister_chrdev_region(radio_first, 1);
        printk(KERN_ALERT DEV_NAME " cannot create nodes\n");
        return -EPERM;
    }

    printk(KERN_ALERT DEV_NAME " created , major number = %d\n", MAJOR(radio_first));

    return 0;
}

void cleanup_module(void)
{
    cdev_del(&radio_dev);
    device_destroy(radio_class, radio_first);
    class_destroy(radio_class);
    unregister_chrdev_region(radio_first, 1);

    printk(KERN_ALERT DEV_NAME " exited\n");
    return ;
}
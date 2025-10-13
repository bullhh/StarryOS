#!/bin/sh

export HOME=/root

echo -e "Welcome to \e[96m\e[1mStarry OS\e[0m!"
env
echo

echo -e "Use \e[1m\e[3mapk\e[0m to install packages."
echo

# Test dma_heap device
echo "Testing DMA heap device..."
if [ -e /dev/dma_heap/system ]; then
    echo "Found /dev/dma_heap/system device"
    ls -l /dev/dma_heap/system
    # Create a simple test script
    cat > /root/test_dma.sh << 'EOF'
#!/bin/sh
echo "Testing DMA heap device access..."
if [ -e /dev/dma_heap/system ]; then
    echo "Device exists"
    # Try to read from the device (this should fail but show up in logs)
    dd if=/dev/dma_heap/system of=/dev/null bs=1 count=1 2>/dev/null || echo "Read failed as expected"
    echo "Test completed"
else
    echo "Device not found"
fi
EOF
    chmod +x /root/test_dma.sh
    echo "Created /root/test_dma.sh for testing"
else
    echo "ERROR: /dev/dma_heap/system device not found"
    ls -l /dev/dma_heap/
    ls -l /dev/
fi

# Do your initialization here!

cd ~
sh --login
#!/bin/bash
echo "🔍 FRAMES-TO-VIDEO DIAGNOSIS"
echo "==============================="
echo ""

echo "1. Checking frames directory..."
echo "   Total frames: $(ls frames/*.png | wc -l)"
echo "   First frame: $(ls frames/*.png | head -1 | xargs basename)"
echo "   Last frame:  $(ls frames/*.png | tail -1 | xargs basename)"
echo "   Timestamp range:"
ls -lt frames/*.png | head -3 | awk '{print "     " $6 " " $9}'
echo ""

echo "2. Frame order check..."
echo "   First 5 frames:"
ls frames/*.png | head -5
echo ""

echo "3. Frame content verification..."
echo "   Frame 0 should be EMPTY (0 bars)"
echo "   Frame 120 should have 10 bars"
echo "   Frame 240 should have 20 bars"
echo ""

echo "4. FFmpeg input pattern..."
echo "   Pattern: frames/frame-%04d.png"
echo "   This expects: frame-0000.png, frame-0001.png, etc."
echo ""

echo "5. FFmpeg frame rate..."
echo "   Input: -r 60 (60 fps)"
echo "   Output: 60 fps (should match)"
echo ""

echo "6. Checking actual frame sizes..."
du -sh frames/ | head -1
echo ""

echo "7. Testing with 3 frames only..."
mkdir -p test-frames
cp frames/frame-0000.png test-frames/
cp frames/frame-0120.png test-frames/
cp frames/frame-0240.png test-frames/
echo "   Created test-frames/ with 3 frames"
echo ""

echo "8. Generating test video..."
ffmpeg -r 60 -i test-frames/frame-%04d.png -c:v libx264 -crf 23 -pix_fmt yuv4202 test-video.mp4 -y 2>&1 | tail -3
echo ""

echo "✅ Test video: test-video.mp4"
echo "   This should show: empty → 10 bars → 20 bars"

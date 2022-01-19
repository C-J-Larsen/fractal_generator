use super::{Color, PixPos};

// Size of BMP file header
const BMP_HEADER_SIZE: usize = 14;
// Size of Device Independent Bitmap header
const DIB_HEADER_SIZE: usize = 40;

pub struct BMPImg<'a> {
    width: u32,
    height: u32,
    file_name: &'a str,
}

// I want to have a function to set the BMP header. I also want to make a
// function that takes a pixel-position-to-pixel-color mapping closure and
// iterates it over all pixels.
impl<'a> BMPImg<'a> {
    pub fn setHeader(&self) {
        let mut bmp_header_data: [u8; BMP_HEADER_SIZE] = [0_u8; BMP_HEADER_SIZE];
        let mut dib_header_data: [u8; DIB_HEADER_SIZE] = [0_u8; DIB_HEADER_SIZE];
        
        // The padding needs to be enough to make pixels (3 bytes each) match
        // 4 byte alignment.
        let padding: u32 = (4 - self.width * 3 % 4) % 4;

        // Calculate size of bitmap with padding
        let bitmapSize: u32 = self.height*(self.width*3 + padding);
        
        // Calculate file size
        let fileSize: u32 = 54 + bitmapSize;
        
        //Write the following to the BMP file in this order:
        //BMP Header:
        //ID ("BM", 2by)
        bmp_header_data[0] = b'B';
        bmp_header_data[1] = b'M';
        
        //File size (4by)
        bmp_header_data[2] = (fileSize & 0xff) as u8;
        bmp_header_data[3] = ((fileSize >> 8) & 0xff) as u8;
        bmp_header_data[4] = ((fileSize >> 16) & 0xff) as u8;
        bmp_header_data[5] = ((fileSize >> 24) & 0xff) as u8;
        
        //Unused bytes (zeroes, 4by)
        bmp_header_data[6] = 0; bmp_header_data[7] = 0;
        bmp_header_data[8] = 0; bmp_header_data[9] = 0;
        
        //Offset to bitmap (0x36, 4by)
        bmp_header_data[10] = 0x36; bmp_header_data[11] = 0;
        bmp_header_data[12] = 0; bmp_header_data[13] =0;
        
        //DIB Header:
        //DIB header size (0x28, 4by)
        dib_header_data[0] = 0x28; dib_header_data[1] = 0;
        dib_header_data[2] = 0; dib_header_data[3] = 0;
        
        //Width of bitmap in pixels (4by)
        dib_header_data[4] = (self.width & 0xff) as u8;
        dib_header_data[5] = ((self.width >> 8) & 0xff) as u8;
        dib_header_data[6] = ((self.width >> 16) & 0xff) as u8;
        dib_header_data[7] = ((self.width >> 24) & 0xff) as u8;
        
        //Height of bitmap in pixels (4by)
        dib_header_data[8] = (self.height & 0xff) as u8;
        dib_header_data[9] = ((self.height >> 8) & 0xff) as u8;
        dib_header_data[10] = ((self.height >> 16) & 0xff) as u8;
        dib_header_data[11] = ((self.height >> 24) & 0xff) as u8;
        
        //Num of color planes (1, 2by)
        dib_header_data[12] = 1; dib_header_data[13] = 0;
        
        //Num of bits per pixel (0x18, 2by)
        dib_header_data[14] = 0x18; dib_header_data[15] = 0;
        
        //BI_RGB (0, 4by)
        dib_header_data[16] = 0; dib_header_data[17] = 0;
        dib_header_data[18] = 0; dib_header_data[19] = 0;
        
        //Size of bitmap w/padding (4)
        dib_header_data[20] = (bitmapSize & 0xff) as u8;
        dib_header_data[21] = ((bitmapSize >> 8) & 0xff) as u8;
        dib_header_data[22] = ((bitmapSize >> 16) & 0xff) as u8;
        dib_header_data[23] = ((bitmapSize >> 24) & 0xff) as u8;
        
        //Print resolution horiz. (0x813, 4by)
        dib_header_data[24] = 0x13; dib_header_data[25] = 0x0B;
        dib_header_data[26] = 0; dib_header_data[27] = 0;
        
        //Print resolution vert. (0x813, 4by)
        dib_header_data[28] = 0x13; dib_header_data[29] = 0x0B;
        dib_header_data[30] = 0; dib_header_data[31] = 0;
        
        //Num of colors (0, 4by)
        dib_header_data[32] = 0; dib_header_data[33] = 0;
        dib_header_data[34] = 0; dib_header_data[35] = 0;
        
        //Num of important colors (0, 4by)
        dib_header_data[36] = 0; dib_header_data[37] = 0;
        dib_header_data[38] = 0; dib_header_data[39] = 0;
        
        // pic.write((char*)dib_header_data.data(), dib_header_data.size());
    }

    pub fn colorPixels(&self, pos_to_color: &dyn Fn(u32, u32) -> Color) {

    }
}

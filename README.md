This is a very simple and easy-to-use color conversion tool that can easily convert colors between Hex, RGB, RGBA, HSL, HSLA, HSV, and CMYK. And each type has its unique API, such as RGB can set color channels, RGBA can set transparency, HSL can set hue, saturation, and brightness, etc.
 ``` rust
    use easy_color::{RGBA, RGB, HSL, Hex, ColorMix};
    use crate::easy_color::{IntoRGB, IntoHex, IntoRGBA, IntoHSL, IntoHSLA, IntoHSV, IntoCMYK};

    let hex:Hex = "#2bc48a".try_into().unwrap();
    
    let mut rgb:RGB = hex.into();
    // or
    let mut rgb = hex.to_rgb();
    assert_eq!(rgb.to_string(), "rgb(43,196,138)");
    rgb.set_red(255);
    assert_eq!(rgb.to_string(), "rgb(255,196,138)");
    
    let mut rgba:RGBA = rgb.into();
    // or
    let mut rgba = rgb.to_rgba();
    rgba.set_alpha(0.5);
    assert_eq!(rgba.to_string(), "rgba(255,196,138,0.50)");
    
    let mut hsl:HSL = rgba.into();
    // or
    let mut hsl = rgba.to_hsl();
    hsl.set_hue(240);
    assert_eq!(hsl.to_string(), "hsl(240,100%,88%)");
    
    let hex:Hex = hsl.into();
    // or
    let hex = hsl.to_hex();
    assert_eq!(hex.to_string(), "#C2C1FF");

    // mix two color
    let hsl:HSL = (0,0,0).try_into().unwrap();
    let rgba:RGBA = (255,255,255,1.0).try_into().unwrap();
    rgba.mix(hsl, None).to_string(); // rgba(127,127,127,1.00)
    rgba.mix(hsl, Some(0.35)).to_string(); // rgba(165,165,165,1.00)//!
    hsl.mix(rgba, None).to_string(); // hsl(0,0%,50%)

    rgba.is_dark(); // false
    rgba.is_light(); // true
    rgba.fade(0.5); // rgba(255,255,255, 0.5)

    // creat random color
    let rgb = RGB::random();
    let rgba = RGBA::random();
    let hsl = HSL::random();

    let hex:Hex = "#2bc48a".try_into().unwrap();
    let hex_str = hex.to_rgb().set_blue(255).to_hsl().set_lightness(50).to_cmyk().set_cyan(100).to_hex().to_string(); // #00B5FF
 ```

 ### Hex
 ``` rust
    let _hex:Hex = "#FAC".try_into().unwrap(); 
    let _hex:Hex = "#FFDFAC".try_into().unwrap();
    let _hex:Hex = "#FFDFACDC".try_into().unwrap() // hex with transparency

    let rgba:RGBA = "rgba(255,223,172,0.85)".try_into().unwrap();
    let hex:Hex = rgba.into();
    let hex_str = hex.to_hex_alpha();
    assert_eq!(hex_str, "#FFDFACD8");

    let hex_str2 = hex.to_hex_alpha();
    assert_eq!(hex_str, "#D8FFDFAC");
 ```
 Convert hex to other types, such as:
 ``` rust
    let hex:Hex = "#FFDFAC".try_into().unwrap();
    let rgb:RGB = hex.into();
    assert_eq!(rgb.to_string(), "rgb(255,223,172)");

    let _hsl:HSL = hex.into();
    let _rgba:RGBA = hex.into();
    let _hsla:HSLA = hex.into();
    let _hsv:HSV = hex.into();
    let _cmyk:CMYK = hex.into();

```

### RGB
RGB can be parsed from a string in the format "rgb(r,g,b)" or from a tuple (r,g,b)
* r:u8 - red value(0~255)
* g:u8 - green value(0~255)
* b:u8 - blue value(0~255)
``` rust
    let mut rgb:RGB = "rgb(43,196,138)".try_into().unwrap();
    rgb.set_green(255);
    assert_eq!(rgb.to_string(), "rgb(43,255,138)");

    let rgb:RGB = (43, 196, 138).try_into().unwrap();
    assert_eq!(rgb.to_string(), "rgb(43,196,138)");
    let hex:Hex = rgb.into();
    assert_eq!(hex.to_string(), "#2BC48A");

    let _rgba:RGBA = rgb.into();
    let _hsl:HSL = rgb.into();
    let _hsla:HSLA = rgb.into();
    let _hsv:HSV = rgb.into();
    let _cmyk:CMYK = rgb.into();

```

### RGBA
RGBA can be parsed from a string in the format "rgba(r,g,b,a)" or from a tuple (r,g,b,a)
* r:u8 - red value(0~255)
* g:u8 - green value(0~255)
* b:u8 - blue value(0~255)
* a:f32 - alpha(0~1)
``` rust
    let mut rgba:RGBA = "rgba(125,60,98,0.8)".try_into().unwrap();
    rgba.set_alpha(0.5);
    assert_eq!(rgba.to_string(), "rgba(125,60,98,0.50)");

    let rgba:RGBA = (125,60,240,0.5).try_into().unwrap();
    let hsl:HSL = rgba.into();
    assert_eq!(hsl.to_string(), "hsl(262,85%,79%)");
```
it also can be convert to other types.

### HSL
HSL can be parsed from a string in the format "hsl(h, s%, l%)" or from a tuple (h,s,l)
* h:u32 - Hue(0~360)
* s:u32 - saturation(0~100)
* l:u32 - lightness(0~100)
``` rust
    let mut hsl:HSL = "hsl(262,85%,79%)".try_into().unwrap();
    hsl.set_lightness(50);
    assert_eq!(hsl.to_string(), "hsl(262,85%,50%)");

    let hsl:HSL = (125,60,75).try_into().unwrap();
    let rgb:RGB = hsl.into();
    assert_eq!(rgb.to_string(), "rgb(153,229,159)")
```

### HSLA
HSLA can be parsed from a string in the format "hsla(h, s%, l%, a)" or from a tuple (h,s,l,a)
* h:u32 - Hue(0~360)
* s:u32 - saturation(0~100)
* l:u32 - lightness(0~100)
* a:f32 - alpha(0~1)
``` rust
    let mut hsla:HSLA = "hsla(262,85%,79%, 0.7)".try_into().unwrap();
    hsla.set_alpha(0.5);
    assert_eq!(hsla.to_string(), "hsla(262,85%,79%,0.50)");

    let hsla:HSLA = (125,60,75,0.6).try_into().unwrap();
    let rgba:RGBA = hsla.into();
    assert_eq!(rgba.to_string(), "rgba(153,229,159,0.60)");
```

### HSV
HSV can be parsed from a string in the format "hsl(h, s%, v%)" or from a tuple (h,s,v). 
* h:u32 - Hue(0~360)
* s:u32 - saturation(0~100)
* v:u32 - Value(0~100)

``` rust
    use easy_color::{RGB, HSV};
    let mut hsv:HSV = "hsv(262,85%,79%)".try_into().unwrap();
    hsv.set_value(50);
    assert_eq!(hsv.to_string(), "hsv(262,85%,50%)");

    let hsv:HSV = (125,60,75).try_into().unwrap();
    assert_eq!(hsv.to_rgb().to_string(), "rgb(76,191,86)");
```

### CMYK
CMYK can be parsed from a string in the format "cmyk(c,m,y,k)" or from a tuple (c,m,y,k). 
* c:u8 - cyan value(0~100)
* m:u8 - magenta value(0~100)
* y:u8 - yellow value(0~100)
* k:u8 - black value(0~100)
``` rust
    use easy_color::{Hex, CMYK};
    let mut cmyk:CMYK = "cmyk(77,34,53,38)".try_into().unwrap();
    cmyk.set_cyan(100);
    assert_eq!(cmyk.to_string(), "cmyk(100,34,53,38)");

    let cmyk:CMYK = (100,34,53,38).try_into().unwrap();
    assert_eq!(cmyk.to_hex().to_string(), "#00684A");
```

### Methods
Each type of structure has the following methods:
``` rust
    fn is_dark(&self) -> bool;
    fn is_light(&self) -> bool;
    fn grayscale(&self) -> Self; 
    fn negate(&self) -> Self;
    fn mix(&self, other:T, weight:Option<f32>) -> Self;
    fn darken(&mut self, ratio:f32) -> Self;
    fn lighten(&mut self, ratio:f32) -> Self;
```

use noise::Noise;

/// A pre-caclulated permutation of 256.
static P: [uint, ..256] = [
    151,  160,  137,  91,   90,   15,   131,  13,   201,  95,
    96,   53,   194,  233,  7,    225,  140,  36,   103,  30,
    69,   142,  8,    99,   37,   240,  21,   10,   23,   190,
    6,    148,  247,  120,  234,  75,   0,    26,   197,  62, 
    94,   252,  219,  203,  117,  35,   11,   32,   57,   177,
    33,   88,   237,  149,  56,   87,   174,  20,   125,  136,
    171,  168,  68,   175,  74,   165,  71,   134,  139,  48,
    27,   166,  77,   146,  158,  231,  83,   111,  229,  122,
    60,   211,  133,  230,  220,  105,  92,   41,   55,   46,
    245,  40,   244,  102,  143,  54,   65,   25,   63,   161,
    1,    216,  80,   73,   209,  76,   132,  187,  208,  89,
    18,   169,  200,  196,  135,  130,  116,  188,  159,  86,
    164,  100,  109,  198,  173,  186,  3,    64,   52,   217,
    226,  250,  124,  123,  5,    202,  38,   147,  118,  126,
    255,  82,   85,   212,  207,  206,  59,   227,  47,   16,
    58,   17,   182,  189,  28,   42,   223,  183,  170,  213,
    119,  248,  152,  2,    44,   154,  163,  70,   221,  153,
    101,  155,  167,  43,   172,  9,    129,  22,   39,   253,
    19,   98,   108,  110,  79,   113,  224,  232,  178,  185,
    112,  104,  218,  246,  97,   228,  251,  34,   242,  193,
    238,  210,  144,  12,   191,  179,  162,  241,  81,   51,
    145,  235,  249,  14,   239,  107,  49,   192,  214,  31,
    181,  199,  106,  157,  184,  84,   204,  176,  115,  121, 
    50,   45,   127,  4,    150,  254,  138,  236,  205,  93,
    222,  114,  67,   29,   24,   72,   243,  141,  128,  195,
    78,   66,   215,  61,   156,  180,  
];                                                                                                                        

/// 3D Perlin noise generator.
pub struct Perlin {
    /// Controls the amount of details.
    octave_count: uint,
    /// The frequency of the first octave.
    frequency: f32,
    /// The frequency multiplier between successive octaves.
    lacuranity: f32,
    /// Controls the roughness.
    persistence: f32,
}

impl Perlin {
    /// Creates a Perlin noise generator with default parameters.
    pub fn new() -> Perlin {
        Perlin {
            octave_count: 6,
            frequency: 1.0,
            persistence: 0.5,
            lacuranity: 2.0,
        }
    }

    /// Sets the number of octaves.
    pub fn set_octave_count(&mut self, n:uint) {
        self.octave_count = n;
    }

    /// Sets the frequency of the first octave.
    pub fn set_frequency(&mut self, frequency:f32) {
        self.frequency = frequency;
    }

    /// Sets the persistence of the signal over succesive octaves.
    pub fn set_persistence(&mut self, persistence:f32) {
        self.persistence = persistence;
    }

    /// Set the frequency multiplier.
    pub fn set_lacuranity(&mut self, lacuranity:f32) {
        self.lacuranity = lacuranity;
    }

    /// Generate one point noise for one octave.
    fn generate_noise(&self, x:f32, y:f32, z:f32) -> f32 {
        // Find integer position of the unit cube that contains point.
        let int_x = x.floor() as uint;
        let int_y = y.floor() as uint;
        let int_z = z.floor() as uint;

        // Move absolute position to cube relative position.
        let x = x - x.floor();
        let y = y - y.floor();
        let z = z - z.floor();

        // Compute S-curves for x, y and z.
        let u = Perlin::fade(x);
        let v = Perlin::fade(y);
        let w = Perlin::fade(z);

        // Find hash coordinates for cube corners. 
        let a =   P[int_x     & 0xFF]+int_y;
        let aa =  P[a         & 0xFF]+int_z;
        let ab =  P[(a+1)     & 0xFF]+int_z;
        let b =   P[(int_x+1) & 0xFF]+int_y;
        let ba =  P[b         & 0xFF]+int_z;
        let bb =  P[(b+1)     & 0xFF]+int_z;

        // Compute gradients and interpolate them in this factorized expression.
        Perlin::lerp(w,
            Perlin::lerp(v,
                Perlin::lerp(u, Perlin::grad(P[aa & 0xFF],   x,    y,    z),
                                Perlin::grad(P[ba & 0xFF],   x-1.0,y,    z)),
                Perlin::lerp(u, Perlin::grad(P[ab & 0xFF],   x,    y-1.0,z),
                                Perlin::grad(P[bb & 0xFF],   x-1.0,y-1.0,z))),
            Perlin::lerp(v,
                Perlin::lerp(u, Perlin::grad(P[(aa+1) & 0xFF], x,    y,    z-1.0),
                                Perlin::grad(P[(ba+1) & 0xFF], x-1.0,y,    z-1.0)),
                Perlin::lerp(u, Perlin::grad(P[(ab+1) & 0xFF], x,    y-1.0,z-1.0),
                                Perlin::grad(P[(bb+1) & 0xFF], x-1.0,y-1.0,z-1.0))))
    }

    /// Compute S-curve.
    fn fade(t:f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Linear interpolation.
    fn lerp(t:f32, a:f32, b:f32) -> f32 {
        a + t * (b - a)
    }

    /// Compute gradient from hash and coordinates.
    fn grad(hash:uint, x:f32, y:f32, z:f32) -> f32 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };
        ( if h&1 == 0 { u } else { -u } ) + ( if h&2 == 0 { v } else { -v } )
    }
}

/// Implements the noise generator common trait.
impl Noise for Perlin {
    /// Returns the noise value at the point(x,y,z)
    /// generated with the current parameters.
    fn get_value(&self, x:f32, y:f32, z:f32) -> f32 {
        // Mutability
        let mut x = x;
        let mut y = y;
        let mut z = z;

        // The computed noise value.
        let mut value = 0.0;
        // The current persistence to decrease between each octave.
        let mut cur_persistence = 1.0;
        // The total amplitude is used to normalize the final value.
        let mut total_amplitude = 0.0;

        // Apply the frequency.
        x *= self.frequency;
        y *= self.frequency;
        z *= self.frequency;

        // For each octave.
        for _ in range(1, self.octave_count + 1) {
            // Compute the noise value.
            value += self.generate_noise(x, y, z) * cur_persistence;
            // Then prepare the next octave.
            x *= self.lacuranity;
            y *= self.lacuranity;
            z *= self.lacuranity;
            total_amplitude += cur_persistence;
            cur_persistence *= self.persistence;
        }
        // Normalize if necessary.
        if value.abs() > 1.0 {
            value /= total_amplitude;
        }
        debug_assert!(value.abs() <= 1.0);
        // The value can be returned here.
        value
    }
}
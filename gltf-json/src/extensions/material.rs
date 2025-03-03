#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
use crate::material::StrengthFactor;
#[cfg(any(
    feature = "KHR_materials_pbrSpecularGlossiness",
    feature = "KHR_materials_transmission"
))]
use crate::texture;
#[cfg(any(
    feature = "KHR_materials_pbrSpecularGlossiness",
    feature = "KHR_materials_transmission",
    feature = "KHR_materials_ior"
))]
use crate::{validation::Validate, Extras};
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Material {
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    #[serde(
        default,
        rename = "KHR_materials_pbrSpecularGlossiness",
        skip_serializing_if = "Option::is_none"
    )]
    pub pbr_specular_glossiness: Option<PbrSpecularGlossiness>,

    #[cfg(feature = "KHR_materials_unlit")]
    #[serde(
        default,
        rename = "KHR_materials_unlit",
        skip_serializing_if = "Option::is_none"
    )]
    pub unlit: Option<Unlit>,

    #[cfg(feature = "KHR_materials_transmission")]
    #[serde(
        default,
        rename = "KHR_materials_transmission",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmission: Option<Transmission>,

    #[cfg(feature = "KHR_materials_volume")]
    #[serde(
        default,
        rename = "KHR_materials_volume",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume: Option<Volume>,

    #[cfg(feature = "KHR_materials_specular")]
    #[serde(
        default,
        rename = "KHR_materials_specular",
        skip_serializing_if = "Option::is_none"
    )]
    pub specular: Option<Specular>,

    #[cfg(feature = "KHR_materials_ior")]
    #[serde(
        default,
        rename = "KHR_materials_ior",
        skip_serializing_if = "Option::is_none"
    )]
    pub ior: Option<Ior>,

    #[cfg(feature = "KHR_materials_clearcoat")]
    #[serde(
        default,
        rename = "KHR_materials_clearcoat",
        skip_serializing_if = "Option::is_none"
    )]
    pub clearcoat: Option<Clearcoat>,

    #[cfg(feature = "OFT_materials_refractive_solid")]
    #[serde(
        default,
        rename = "OFT_materials_refractive_solid",
        skip_serializing_if = "Option::is_none"
    )]
    pub refractive_solid: Option<RefractiveSolid>,
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PbrMetallicRoughness {}

/// A set of parameter values that are used to define the specular-glossiness
/// material model from Physically-Based Rendering (PBR) methodology.
///
/// This model supports more materials than metallic-roughness, at the cost of
/// increased memory use. When both are available, specular-glossiness should be
/// preferred.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct PbrSpecularGlossiness {
    /// The material's diffuse factor.
    ///
    /// The RGBA components of the reflected diffuse color of the
    /// material. Metals have a diffuse value of `[0.0, 0.0, 0.0]`. The fourth
    /// component (A) is the alpha coverage of the material. The `alphaMode`
    /// property specifies how alpha is interpreted. The values are linear.
    pub diffuse_factor: PbrDiffuseFactor,

    /// The diffuse texture.
    ///
    /// This texture contains RGB(A) components of the reflected diffuse color
    /// of the material in sRGB color space. If the fourth component (A) is
    /// present, it represents the alpha coverage of the material. Otherwise, an
    /// alpha of 1.0 is assumed. The `alphaMode` property specifies how alpha is
    /// interpreted. The stored texels must not be premultiplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse_texture: Option<texture::Info>,

    /// The material's specular factor.
    pub specular_factor: PbrSpecularFactor,

    /// The glossiness or smoothness of the material.
    ///
    /// A value of 1.0 means the material has full glossiness or is perfectly
    /// smooth. A value of 0.0 means the material has no glossiness or is
    /// completely rough. This value is linear.
    pub glossiness_factor: StrengthFactor,

    /// The specular-glossiness texture.
    ///
    /// A RGBA texture, containing the specular color of the material (RGB
    /// components) and its glossiness (A component). The values are in sRGB
    /// space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_glossiness_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct NormalTexture {
    #[cfg(feature = "OFT_texture_highPrecisionNormal")]
    #[serde(
        default,
        rename = "OFT_texture_highPrecisionNormal",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_precision_normal: Option<HighPrecisionNormal>,
}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OcclusionTexture {}

/// The diffuse factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrDiffuseFactor(pub [f32; 4]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrDiffuseFactor {
    fn default() -> Self {
        PbrDiffuseFactor([1.0, 1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrDiffuseFactor {}

/// The specular factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrSpecularFactor(pub [f32; 3]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrSpecularFactor {
    fn default() -> Self {
        PbrSpecularFactor([1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrSpecularFactor {}

/// Empty struct that should be present for primitives which should not be shaded with the PBR shading model.
#[cfg(feature = "KHR_materials_unlit")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Unlit {}

/// A number in the inclusive range [0.0, 1.0] with a default value of 0.0.
#[cfg(feature = "KHR_materials_transmission")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TransmissionFactor(pub f32);

#[cfg(feature = "KHR_materials_transmission")]
impl Default for TransmissionFactor {
    fn default() -> Self {
        TransmissionFactor(0.0)
    }
}

#[cfg(feature = "KHR_materials_transmission")]
impl Validate for TransmissionFactor {}

#[cfg(feature = "KHR_materials_transmission")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Transmission {
    /// The base percentage of light that is transmitted through the surface.
    ///
    /// The amount of light that is transmitted by the surface rather than diffusely re-emitted.
    /// This is a percentage of all the light that penetrates a surface (i.e. isn’t specularly reflected)
    /// rather than a percentage of the total light that hits a surface.
    /// A value of 1.0 means that 100% of the light that penetrates the surface is transmitted through.
    pub transmission_factor: TransmissionFactor,

    /// The transmission texture.
    ///
    /// The R channel of this texture defines the amount of light that is transmitted by the surface
    /// rather than diffusely re-emitted. A value of 1.0 in the red channel means that 100% of the light
    /// that penetrates the surface (i.e. isn’t specularly reflected) is transmitted through.
    /// The value is linear and is multiplied by the transmissionFactor to determine the total transmission value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// A positive number with default value of 1.5
#[cfg(feature = "KHR_materials_ior")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IndexOfRefraction(pub f32);

#[cfg(feature = "KHR_materials_ior")]
impl Default for IndexOfRefraction {
    fn default() -> Self {
        IndexOfRefraction(1.5)
    }
}

#[cfg(feature = "KHR_materials_ior")]
impl Validate for IndexOfRefraction {}

#[cfg(feature = "KHR_materials_ior")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Ior {
    /// The index of refraction.
    ///
    /// Typical values for the index of refraction range from 1 to 2.
    /// In rare cases values greater than 2 are possible.
    /// For example, the ior of water is 1.33, and diamond is 2.42
    pub ior: IndexOfRefraction,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// A number in the inclusive range [0.0, +inf] with a default value of 0.0.
#[cfg(feature = "KHR_materials_volume")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ThicknessFactor(pub f32);

#[cfg(feature = "KHR_materials_volume")]
impl Default for ThicknessFactor {
    fn default() -> Self {
        ThicknessFactor(0.0)
    }
}

#[cfg(feature = "KHR_materials_volume")]
impl Validate for ThicknessFactor {}

/// A number in the inclusive range [0.0, +inf] with a default value of +inf.
#[cfg(feature = "KHR_materials_volume")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AttenuationDistance(pub f32);

#[cfg(feature = "KHR_materials_volume")]
impl Default for AttenuationDistance {
    fn default() -> Self {
        AttenuationDistance(f32::INFINITY)
    }
}

#[cfg(feature = "KHR_materials_volume")]
impl Validate for AttenuationDistance {}

/// A colour in the inclusive range [[0.0; 3], [1.0; 3]] with a default value of [1.0; 3].
#[cfg(feature = "KHR_materials_volume")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AttenuationColor(pub [f32; 3]);

#[cfg(feature = "KHR_materials_volume")]
impl Default for AttenuationColor {
    fn default() -> Self {
        AttenuationColor([1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_volume")]
impl Validate for AttenuationColor {}

#[cfg(feature = "KHR_materials_volume")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Volume {
    /// The thickness of the volume beneath the surface. The value is
    /// given in the coordinate space of the mesh. If the value is 0
    /// the material is thin-walled. Otherwise the material is a
    /// volume boundary. The `doubleSided` property has no effect on
    /// volume boundaries. Range is [0, +inf).
    pub thickness_factor: ThicknessFactor,

    /// A texture that defines the thickness, stored in the G channel.
    /// This will be multiplied by `thickness_factor`. Range is [0, 1].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness_texture: Option<texture::Info>,

    /// Density of the medium given as the average distance that light
    /// travels in the medium before interacting with a particle. The
    /// value is given in world space. Range is (0, +inf).
    pub attenuation_distance: AttenuationDistance,

    /// The color that white light turns into due to absorption when
    /// reaching the attenuation distance.
    pub attenuation_color: AttenuationColor,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// A number in the inclusive range [0.0, +inf] with a default value of 1.0.
#[cfg(feature = "KHR_materials_specular")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SpecularFactor(pub f32);

#[cfg(feature = "KHR_materials_specular")]
impl Default for SpecularFactor {
    fn default() -> Self {
        SpecularFactor(1.0)
    }
}

#[cfg(feature = "KHR_materials_specular")]
impl Validate for SpecularFactor {}

/// A colour in the inclusive range [[0.0; 3], [1.0; 3]] with a default value of [1.0; 3].
#[cfg(feature = "KHR_materials_specular")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SpecularColorFactor(pub [f32; 3]);

#[cfg(feature = "KHR_materials_specular")]
impl Default for SpecularColorFactor {
    fn default() -> Self {
        SpecularColorFactor([1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_specular")]
impl Validate for SpecularColorFactor {}

#[cfg(feature = "KHR_materials_specular")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Specular {
    /// The strength of the specular reflection.
    pub specular_factor: SpecularFactor,

    /// A texture that defines the strength of the specular reflection,
    /// stored in the alpha (`A`) channel. This will be multiplied by
    /// `specular_factor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_texture: Option<texture::Info>,

    /// The F0 color of the specular reflection (linear RGB).
    pub specular_color_factor: SpecularColorFactor,

    /// A texture that defines the F0 color of the specular reflection,
    /// stored in the `RGB` channels and encoded in sRGB. This texture
    /// will be multiplied by `specular_color_factor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_color_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

#[cfg(feature = "KHR_materials_clearcoat")]
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct ClearcoatFactor(pub f32);

#[cfg(feature = "KHR_materials_clearcoat")]
impl Validate for ClearcoatFactor {}

#[cfg(feature = "KHR_materials_clearcoat")]
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct ClearcoatRoughnessFactor(pub f32);

#[cfg(feature = "KHR_materials_clearcoat")]
impl Validate for ClearcoatRoughnessFactor {}

#[cfg(feature = "KHR_materials_clearcoat")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct Clearcoat {
    /// The clearcoat layer intensity.
    pub clearcoat_factor: ClearcoatFactor,
    /// The clearcoat layer intensity texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearcoat_texture: Option<texture::Info>,
    /// The clearcoat layer roughness.
    pub clearcoat_roughness_factor: ClearcoatRoughnessFactor,
    /// The clearcoat layer roughness texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearcoat_roughness_texture: Option<texture::Info>,
    /// The clearcoat normal map texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearcoat_normal_texture: Option<texture::Info>,
}

#[cfg(feature = "OFT_materials_refractive_solid")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct RefractiveSolid {
    /// Specify mesh proxy of this node.
    pub geometry_proxy: String,
    /// Intensity of envmap lighting.
    pub env_map_intensity: f32,
    /// Reflectivity.
    pub reflectivity: f32,
    /// Refractive index.
    pub refractive_index: f32,
    /// Control dispersion.
    pub r_index_delta: f32,
    /// Specify how much light would be absorbed in traveling.
    pub absorbption: [f32; 3],
    /// Speed of absorbption.
    pub absorbption_factor: f32,
}

#[cfg(feature = "OFT_materials_refractive_solid")]
impl Default for RefractiveSolid {
    fn default() -> Self {
        RefractiveSolid {
            geometry_proxy: String::new(),
            env_map_intensity: 1.0,
            reflectivity: 1.0,
            refractive_index: 1.0,
            r_index_delta: 0.1,
            absorbption: [1.0, 1.0, 1.0],
            absorbption_factor: 1.0,
        }
    }
}

#[cfg(feature = "OFT_texture_highPrecisionNormal")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct HighPrecisionNormal {
    #[serde(rename = "lower8BitTexture")]
    pub lower_8_bit_texture: crate::texture::Info,
}

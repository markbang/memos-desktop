#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`Attachment`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "filename",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "description": "Input only. The content of the attachment.",
    ///      "writeOnly": true,
    ///      "type": "string",
    ///      "format": "bytes"
    ///    },
    ///    "createTime": {
    ///      "description": "Output only. The creation timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "externalLink": {
    ///      "description": "Optional. The external link of the attachment.",
    ///      "type": "string"
    ///    },
    ///    "filename": {
    ///      "description": "The filename of the attachment.",
    ///      "type": "string"
    ///    },
    ///    "mediaMetadata": {
    ///      "description": "Optional. Immutable normalized media metadata
    /// explicitly supplied by the client at creation time.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MediaMetadata"
    ///        }
    ///      ]
    ///    },
    ///    "memo": {
    ///      "description": "Optional. The related memo. Refer to `Memo.name`.\n
    /// Format: memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "motionMedia": {
    ///      "description": "Optional. Motion media metadata.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MotionMedia"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "The name of the attachment.\n Format:
    /// attachments/{attachment}",
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "description": "Output only. The size of the attachment in bytes.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "The MIME type of the attachment.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Attachment {
        ///Input only. The content of the attachment.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        ///Output only. The creation timestamp.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Optional. The external link of the attachment.
        #[serde(
            rename = "externalLink",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub external_link: ::std::option::Option<::std::string::String>,
        ///The filename of the attachment.
        pub filename: ::std::string::String,
        ///Optional. Immutable normalized media metadata explicitly supplied by
        /// the client at creation time.
        #[serde(
            rename = "mediaMetadata",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub media_metadata: ::std::option::Option<MediaMetadata>,
        ///Optional. The related memo. Refer to `Memo.name`.
        /// Format: memos/{memo}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub memo: ::std::option::Option<::std::string::String>,
        ///Optional. Motion media metadata.
        #[serde(
            rename = "motionMedia",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub motion_media: ::std::option::Option<MotionMedia>,
        ///The name of the attachment.
        /// Format: attachments/{attachment}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Output only. The size of the attachment in bytes.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
        ///The MIME type of the attachment.
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    ///`BatchDeleteAttachmentsRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "names"
    ///  ],
    ///  "properties": {
    ///    "names": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchDeleteAttachmentsRequest {
        pub names: ::std::vec::Vec<::std::string::String>,
    }

    ///Request message for BatchGetInstanceSettings method.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Request message for BatchGetInstanceSettings method.",
    ///  "type": "object",
    ///  "required": [
    ///    "names"
    ///  ],
    ///  "properties": {
    ///    "names": {
    ///      "description": "The resource names of the instance settings.\n
    /// Format: instance/settings/{setting}",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchGetInstanceSettingsRequest {
        ///The resource names of the instance settings.
        /// Format: instance/settings/{setting}
        pub names: ::std::vec::Vec<::std::string::String>,
    }

    ///Response message for BatchGetInstanceSettings method.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response message for BatchGetInstanceSettings method.",
    ///  "type": "object",
    ///  "properties": {
    ///    "settings": {
    ///      "description": "The instance settings in the same order as the
    /// input names.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceSetting"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchGetInstanceSettingsResponse {
        ///The instance settings in the same order as the input names.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub settings: ::std::vec::Vec<InstanceSetting>,
    }

    impl ::std::default::Default for BatchGetInstanceSettingsResponse {
        fn default() -> Self {
            Self {
                settings: Default::default(),
            }
        }
    }

    ///`BatchGetLinkMetadataRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "urls"
    ///  ],
    ///  "properties": {
    ///    "urls": {
    ///      "description": "Required. The link URLs.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchGetLinkMetadataRequest {
        ///Required. The link URLs.
        pub urls: ::std::vec::Vec<::std::string::String>,
    }

    ///`BatchGetLinkMetadataResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "linkMetadata": {
    ///      "description": "The link metadata list, in the same order as the
    /// input URLs.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/LinkMetadata"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchGetLinkMetadataResponse {
        ///The link metadata list, in the same order as the input URLs.
        #[serde(
            rename = "linkMetadata",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub link_metadata: ::std::vec::Vec<LinkMetadata>,
    }

    impl ::std::default::Default for BatchGetLinkMetadataResponse {
        fn default() -> Self {
            Self {
                link_metadata: Default::default(),
            }
        }
    }

    ///`BatchGetUsersRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "usernames": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchGetUsersRequest {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub usernames: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::default::Default for BatchGetUsersRequest {
        fn default() -> Self {
            Self {
                usernames: Default::default(),
            }
        }
    }

    ///`BatchGetUsersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "users": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/User"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BatchGetUsersResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub users: ::std::vec::Vec<User>,
    }

    impl ::std::default::Default for BatchGetUsersResponse {
        fn default() -> Self {
            Self {
                users: Default::default(),
            }
        }
    }

    ///Represents a color in the RGBA color space. This representation is
    /// designed for simplicity of conversion to/from color representations
    /// in various languages over compactness. For example, the fields of
    /// this representation can be trivially provided to the constructor of
    /// `java.awt.Color` in Java; it can also be trivially provided to
    /// UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with
    /// just a little work, it can be easily formatted into a CSS `rgba()`
    /// string in JavaScript.
    ///
    /// This reference page doesn't carry information about the absolute color
    /// space
    /// that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB,
    /// DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB
    /// color space.
    ///
    /// When color equality needs to be decided, implementations, unless
    /// documented otherwise, treat two colors as equal if all their red,
    /// green, blue, and alpha values each differ by at most 1e-5.
    ///
    /// Example (Java):
    ///
    ///      import com.google.type.Color;
    ///
    ///      // ...
    ///      public static java.awt.Color fromProto(Color protocolor) {
    ///        float alpha = protocolor.hasAlpha()
    ///            ? protocolor.getAlpha().getValue()
    ///            : 1.0;
    ///
    ///        return new java.awt.Color(
    ///            protocolor.getRed(),
    ///            protocolor.getGreen(),
    ///            protocolor.getBlue(),
    ///            alpha);
    ///      }
    ///
    ///      public static Color toProto(java.awt.Color color) {
    ///        float red = (float) color.getRed();
    ///        float green = (float) color.getGreen();
    ///        float blue = (float) color.getBlue();
    ///        float denominator = 255.0;
    ///        Color.Builder resultBuilder =
    ///            Color
    ///                .newBuilder()
    ///                .setRed(red / denominator)
    ///                .setGreen(green / denominator)
    ///                .setBlue(blue / denominator);
    ///        int alpha = color.getAlpha();
    ///        if (alpha != 255) {
    ///          result.setAlpha(
    ///              FloatValue
    ///                  .newBuilder()
    ///                  .setValue(((float) alpha) / denominator)
    ///                  .build());
    ///        }
    ///        return resultBuilder.build();
    ///      }
    ///      // ...
    ///
    /// Example (iOS / Obj-C):
    ///
    ///      // ...
    ///      static UIColor* fromProto(Color* protocolor) {
    ///         float red = [protocolor red];
    ///         float green = [protocolor green];
    ///         float blue = [protocolor blue];
    ///         FloatValue* alpha_wrapper = [protocolor alpha];
    ///         float alpha = 1.0;
    ///         if (alpha_wrapper != nil) {
    ///           alpha = [alpha_wrapper value];
    ///         }
    ///         return [UIColor colorWithRed:red green:green blue:blue
    /// alpha:alpha];      }
    ///
    ///      static Color* toProto(UIColor* color) {
    ///          CGFloat red, green, blue, alpha;
    ///          if (![color getRed:&red green:&green blue:&blue alpha:&alpha])
    /// {            return nil;
    ///          }
    ///          Color* result = [[Color alloc] init];
    ///          [result setRed:red];
    ///          [result setGreen:green];
    ///          [result setBlue:blue];
    ///          if (alpha <= 0.9999) {
    ///            [result setAlpha:floatWrapperWithValue(alpha)];
    ///          }
    ///          [result autorelease];
    ///          return result;
    ///     }
    ///     // ...
    ///
    ///  Example (JavaScript):
    ///
    ///     // ...
    ///
    ///     var protoToCssColor = function(rgb_color) {
    ///        var redFrac = rgb_color.red || 0.0;
    ///        var greenFrac = rgb_color.green || 0.0;
    ///        var blueFrac = rgb_color.blue || 0.0;
    ///        var red = Math.floor(redFrac * 255);
    ///        var green = Math.floor(greenFrac * 255);
    ///        var blue = Math.floor(blueFrac * 255);
    ///
    ///        if (!('alpha' in rgb_color)) {
    ///           return rgbToCssColor(red, green, blue);
    ///        }
    ///
    ///        var alphaFrac = rgb_color.alpha.value || 0.0;
    ///        var rgbParams = [red, green, blue].join(',');
    ///        return ['rgba(', rgbParams, ',', alphaFrac, ')'].join('');
    ///     };
    ///
    ///     var rgbToCssColor = function(red, green, blue) {
    ///       var rgbNumber = new Number((red << 16) | (green << 8) | blue);
    ///       var hexString = rgbNumber.toString(16);
    ///       var missingZeros = 6 - hexString.length;
    ///       var resultBuilder = ['#'];
    ///       for (var i = 0; i < missingZeros; i++) {
    ///          resultBuilder.push('0');
    ///       }
    ///       resultBuilder.push(hexString);
    ///       return resultBuilder.join('');
    ///     };
    ///
    ///     // ...
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Represents a color in the RGBA color space. This
    /// representation is designed\n for simplicity of conversion to/from color
    /// representations in various\n languages over compactness. For example,
    /// the fields of this representation\n can be trivially provided to the
    /// constructor of `java.awt.Color` in Java; it\n can also be trivially
    /// provided to UIColor's `+colorWithRed:green:blue:alpha`\n method in iOS;
    /// and, with just a little work, it can be easily formatted into\n a CSS
    /// `rgba()` string in JavaScript.\n\n This reference page doesn't carry
    /// information about the absolute color\n space\n that should be used to
    /// interpret the RGB value (e.g. sRGB, Adobe RGB,\n DCI-P3, BT.2020, etc.).
    /// By default, applications should assume the sRGB color\n space.\n\n When
    /// color equality needs to be decided, implementations, unless\n documented
    /// otherwise, treat two colors as equal if all their red,\n green, blue,
    /// and alpha values each differ by at most 1e-5.\n\n Example (Java):\n\n
    /// import com.google.type.Color;\n\n      // ...\n      public static
    /// java.awt.Color fromProto(Color protocolor) {\n        float alpha =
    /// protocolor.hasAlpha()\n            ? protocolor.getAlpha().getValue()\n
    /// : 1.0;\n\n        return new java.awt.Color(\n
    /// protocolor.getRed(),\n            protocolor.getGreen(),\n
    /// protocolor.getBlue(),\n            alpha);\n      }\n\n      public
    /// static Color toProto(java.awt.Color color) {\n        float red =
    /// (float) color.getRed();\n        float green = (float)
    /// color.getGreen();\n        float blue = (float) color.getBlue();\n
    /// float denominator = 255.0;\n        Color.Builder resultBuilder =\n
    /// Color\n                .newBuilder()\n                .setRed(red /
    /// denominator)\n                .setGreen(green / denominator)\n
    /// .setBlue(blue / denominator);\n        int alpha = color.getAlpha();\n
    /// if (alpha != 255) {\n          result.setAlpha(\n
    /// FloatValue\n                  .newBuilder()\n
    /// .setValue(((float) alpha) / denominator)\n                  .build());\n
    /// }\n        return resultBuilder.build();\n      }\n      // ...\n\n
    /// Example (iOS / Obj-C):\n\n      // ...\n      static UIColor*
    /// fromProto(Color* protocolor) {\n         float red = [protocolor red];\n
    /// float green = [protocolor green];\n         float blue = [protocolor
    /// blue];\n         FloatValue* alpha_wrapper = [protocolor alpha];\n
    /// float alpha = 1.0;\n         if (alpha_wrapper != nil) {\n
    /// alpha = [alpha_wrapper value];\n         }\n         return [UIColor
    /// colorWithRed:red green:green blue:blue alpha:alpha];\n      }\n\n
    /// static Color* toProto(UIColor* color) {\n          CGFloat red, green,
    /// blue, alpha;\n          if (![color getRed:&red green:&green blue:&blue
    /// alpha:&alpha]) {\n            return nil;\n          }\n          Color*
    /// result = [[Color alloc] init];\n          [result setRed:red];\n
    /// [result setGreen:green];\n          [result setBlue:blue];\n          if
    /// (alpha <= 0.9999) {\n            [result
    /// setAlpha:floatWrapperWithValue(alpha)];\n          }\n          [result
    /// autorelease];\n          return result;\n     }\n     // ...\n\n
    /// Example (JavaScript):\n\n     // ...\n\n     var protoToCssColor =
    /// function(rgb_color) {\n        var redFrac = rgb_color.red || 0.0;\n
    /// var greenFrac = rgb_color.green || 0.0;\n        var blueFrac =
    /// rgb_color.blue || 0.0;\n        var red = Math.floor(redFrac * 255);\n
    /// var green = Math.floor(greenFrac * 255);\n        var blue =
    /// Math.floor(blueFrac * 255);\n\n        if (!('alpha' in rgb_color)) {\n
    /// return rgbToCssColor(red, green, blue);\n        }\n\n        var
    /// alphaFrac = rgb_color.alpha.value || 0.0;\n        var rgbParams = [red,
    /// green, blue].join(',');\n        return ['rgba(', rgbParams, ',',
    /// alphaFrac, ')'].join('');\n     };\n\n     var rgbToCssColor =
    /// function(red, green, blue) {\n       var rgbNumber = new Number((red <<
    /// 16) | (green << 8) | blue);\n       var hexString =
    /// rgbNumber.toString(16);\n       var missingZeros = 6 -
    /// hexString.length;\n       var resultBuilder = ['#'];\n       for (var i
    /// = 0; i < missingZeros; i++) {\n          resultBuilder.push('0');\n
    /// }\n       resultBuilder.push(hexString);\n       return
    /// resultBuilder.join('');\n     };\n\n     // ...",
    ///  "type": "object",
    ///  "properties": {
    ///    "alpha": {
    ///      "description": "The fraction of this color that should be applied
    /// to the pixel. That is,\n the final pixel color is defined by the
    /// equation:\n\n   `pixel color = alpha * (this color) + (1.0 - alpha) *
    /// (background color)`\n\n This means that a value of 1.0 corresponds to a
    /// solid color, whereas\n a value of 0.0 corresponds to a completely
    /// transparent color. This\n uses a wrapper message rather than a simple
    /// float scalar so that it is\n possible to distinguish between a default
    /// value and the value being unset.\n If omitted, this color object is
    /// rendered as a solid color\n (as if the alpha value had been explicitly
    /// given a value of 1.0).",
    ///      "type": "number",
    ///      "format": "float"
    ///    },
    ///    "blue": {
    ///      "description": "The amount of blue in the color as a value in the
    /// interval [0, 1].",
    ///      "type": "number",
    ///      "format": "float"
    ///    },
    ///    "green": {
    ///      "description": "The amount of green in the color as a value in the
    /// interval [0, 1].",
    ///      "type": "number",
    ///      "format": "float"
    ///    },
    ///    "red": {
    ///      "description": "The amount of red in the color as a value in the
    /// interval [0, 1].",
    ///      "type": "number",
    ///      "format": "float"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Color {
        ///The fraction of this color that should be applied to the pixel. That
        /// is, the final pixel color is defined by the equation:
        ///
        ///   `pixel color = alpha * (this color) + (1.0 - alpha) * (background
        /// color)`
        ///
        /// This means that a value of 1.0 corresponds to a solid color, whereas
        /// a value of 0.0 corresponds to a completely transparent color. This
        /// uses a wrapper message rather than a simple float scalar so that it
        /// is possible to distinguish between a default value and the
        /// value being unset. If omitted, this color object is rendered
        /// as a solid color (as if the alpha value had been explicitly
        /// given a value of 1.0).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha: ::std::option::Option<f32>,
        ///The amount of blue in the color as a value in the interval [0, 1].
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub blue: ::std::option::Option<f32>,
        ///The amount of green in the color as a value in the interval [0, 1].
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub green: ::std::option::Option<f32>,
        ///The amount of red in the color as a value in the interval [0, 1].
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub red: ::std::option::Option<f32>,
    }

    impl ::std::default::Default for Color {
        fn default() -> Self {
            Self {
                alpha: Default::default(),
                blue: Default::default(),
                green: Default::default(),
                red: Default::default(),
            }
        }
    }

    ///`CreateLinkedIdentityRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "idpName",
    ///    "parent",
    ///    "redirectUri"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "description": "Required. The authorization code from the identity
    /// provider.",
    ///      "type": "string"
    ///    },
    ///    "codeVerifier": {
    ///      "description": "Optional. The PKCE code verifier used in the OAuth
    /// flow.",
    ///      "type": "string"
    ///    },
    ///    "idpName": {
    ///      "description": "Required. The identity provider to link.\n Format:
    /// identity-providers/{idp}",
    ///      "type": "string"
    ///    },
    ///    "parent": {
    ///      "description": "Required. The parent user who owns the linked
    /// identity.\n Format: users/{user}",
    ///      "type": "string"
    ///    },
    ///    "redirectUri": {
    ///      "description": "Required. The redirect URI used in the OAuth
    /// flow.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateLinkedIdentityRequest {
        ///Required. The authorization code from the identity provider.
        pub code: ::std::string::String,
        ///Optional. The PKCE code verifier used in the OAuth flow.
        #[serde(
            rename = "codeVerifier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub code_verifier: ::std::option::Option<::std::string::String>,
        ///Required. The identity provider to link.
        /// Format: identity-providers/{idp}
        #[serde(rename = "idpName")]
        pub idp_name: ::std::string::String,
        ///Required. The parent user who owns the linked identity.
        /// Format: users/{user}
        pub parent: ::std::string::String,
        ///Required. The redirect URI used in the OAuth flow.
        #[serde(rename = "redirectUri")]
        pub redirect_uri: ::std::string::String,
    }

    ///`CreatePersonalAccessTokenRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "parent"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "Optional. Description of the personal access
    /// token.",
    ///      "type": "string"
    ///    },
    ///    "expiresInDays": {
    ///      "description": "Optional. Expiration duration in days (0 = never
    /// expires).",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "parent": {
    ///      "description": "Required. The parent resource where this token will
    /// be created.\n Format: users/{user}",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreatePersonalAccessTokenRequest {
        ///Optional. Description of the personal access token.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Optional. Expiration duration in days (0 = never expires).
        #[serde(
            rename = "expiresInDays",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expires_in_days: ::std::option::Option<i32>,
        ///Required. The parent resource where this token will be created.
        /// Format: users/{user}
        pub parent: ::std::string::String,
    }

    ///`CreatePersonalAccessTokenResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "personalAccessToken": {
    ///      "description": "The personal access token metadata.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PersonalAccessToken"
    ///        }
    ///      ]
    ///    },
    ///    "token": {
    ///      "description": "The actual token value - only returned on
    /// creation.\n This is the only time the token value will be visible.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreatePersonalAccessTokenResponse {
        ///The personal access token metadata.
        #[serde(
            rename = "personalAccessToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub personal_access_token: ::std::option::Option<PersonalAccessToken>,
        ///The actual token value - only returned on creation.
        /// This is the only time the token value will be visible.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for CreatePersonalAccessTokenResponse {
        fn default() -> Self {
            Self {
                personal_access_token: Default::default(),
                token: Default::default(),
            }
        }
    }

    ///`FieldMapping`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "avatarUrl": {
    ///      "type": "string"
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "identifier": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FieldMapping {
        #[serde(
            rename = "avatarUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub identifier: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for FieldMapping {
        fn default() -> Self {
            Self {
                avatar_url: Default::default(),
                display_name: Default::default(),
                email: Default::default(),
                identifier: Default::default(),
            }
        }
    }

    ///Custom profile configuration for instance branding.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Custom profile configuration for instance branding.",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "logoUrl": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GeneralSettingCustomProfile {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "logoUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub logo_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for GeneralSettingCustomProfile {
        fn default() -> Self {
            Self {
                description: Default::default(),
                logo_url: Default::default(),
                title: Default::default(),
            }
        }
    }

    ///`GetCurrentUserResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "user": {
    ///      "description": "The authenticated user's information.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/User"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetCurrentUserResponse {
        ///The authenticated user's information.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<User>,
    }

    impl ::std::default::Default for GetCurrentUserResponse {
        fn default() -> Self {
            Self {
                user: Default::default(),
            }
        }
    }

    ///`GetUserWebhookSigningSecretResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "signingSecret": {
    ///      "description": "The signing secret, in the Standard Webhooks
    /// \"whsec_<base64>\" form.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetUserWebhookSigningSecretResponse {
        ///The signing secret, in the Standard Webhooks "whsec_<base64>" form.
        #[serde(
            rename = "signingSecret",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub signing_secret: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for GetUserWebhookSigningSecretResponse {
        fn default() -> Self {
            Self {
                signing_secret: Default::default(),
            }
        }
    }

    ///Contains an arbitrary serialized message along with a @type that
    /// describes the type of the serialized message.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains an arbitrary serialized message along with a
    /// @type that describes the type of the serialized message.",
    ///  "type": "object",
    ///  "properties": {
    ///    "@type": {
    ///      "description": "The type of the serialized message.",
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GoogleProtobufAny {
        ///The type of the serialized message.
        #[serde(
            rename = "@type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for GoogleProtobufAny {
        fn default() -> Self {
            Self {
                type_: Default::default(),
            }
        }
    }

    ///`IdentityProvider`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "config",
    ///    "title",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "config": {
    ///      "description": "Required. Configuration for the identity
    /// provider.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IdentityProviderConfig"
    ///        }
    ///      ]
    ///    },
    ///    "identifierFilter": {
    ///      "description": "Optional. Filter applied to user identifiers.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the identity provider.\n
    /// Format: identity-providers/{idp}",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "Required. The display title of the identity
    /// provider.",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "Required. The type of the identity provider.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "TYPE_UNSPECIFIED",
    ///        "OAUTH2"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityProvider {
        ///Required. Configuration for the identity provider.
        pub config: IdentityProviderConfig,
        ///Optional. Filter applied to user identifiers.
        #[serde(
            rename = "identifierFilter",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub identifier_filter: ::std::option::Option<::std::string::String>,
        ///The resource name of the identity provider.
        /// Format: identity-providers/{idp}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Required. The display title of the identity provider.
        pub title: ::std::string::String,
        ///Required. The type of the identity provider.
        #[serde(rename = "type")]
        pub type_: IdentityProviderType,
    }

    ///`IdentityProviderConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "oauth2Config": {
    ///      "$ref": "#/components/schemas/OAuth2Config"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityProviderConfig {
        #[serde(
            rename = "oauth2Config",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub oauth2_config: ::std::option::Option<OAuth2Config>,
    }

    impl ::std::default::Default for IdentityProviderConfig {
        fn default() -> Self {
            Self {
                oauth2_config: Default::default(),
            }
        }
    }

    ///Required. The type of the identity provider.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Required. The type of the identity provider.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "TYPE_UNSPECIFIED",
    ///    "OAUTH2"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum IdentityProviderType {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        TypeUnspecified,
        #[serde(rename = "OAUTH2")]
        Oauth2,
    }

    impl ::std::fmt::Display for IdentityProviderType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::TypeUnspecified => f.write_str("TYPE_UNSPECIFIED"),
                Self::Oauth2 => f.write_str("OAUTH2"),
            }
        }
    }

    impl ::std::str::FromStr for IdentityProviderType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TYPE_UNSPECIFIED" => Ok(Self::TypeUnspecified),
                "OAUTH2" => Ok(Self::Oauth2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Instance profile message containing basic instance information.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Instance profile message containing basic instance
    /// information.",
    ///  "type": "object",
    ///  "properties": {
    ///    "admin": {
    ///      "description": "The first administrator who set up this instance,
    /// for display purposes.\n May be null on an instance that has lost all
    /// admins; use needs_setup to\n determine whether initial setup is actually
    /// required.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/User"
    ///        }
    ///      ]
    ///    },
    ///    "commit": {
    ///      "description": "Commit is the current build commit of instance.",
    ///      "type": "string"
    ///    },
    ///    "demo": {
    ///      "description": "Demo indicates if the instance is in demo mode.",
    ///      "type": "boolean"
    ///    },
    ///    "instanceUrl": {
    ///      "description": "Instance URL is the URL of the instance.",
    ///      "type": "string"
    ///    },
    ///    "needsSetup": {
    ///      "description": "NeedsSetup is true when the instance has no users
    /// yet and requires initial\n setup (creating the first admin account).
    /// Unlike a null admin, this stays\n false once any user exists, so an
    /// instance that has lost its admins is not\n mistaken for a fresh
    /// install.",
    ///      "type": "boolean"
    ///    },
    ///    "version": {
    ///      "description": "Version is the current version of instance.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceProfile {
        ///The first administrator who set up this instance, for display
        /// purposes. May be null on an instance that has lost all
        /// admins; use needs_setup to determine whether initial setup
        /// is actually required.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub admin: ::std::option::Option<User>,
        ///Commit is the current build commit of instance.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit: ::std::option::Option<::std::string::String>,
        ///Demo indicates if the instance is in demo mode.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub demo: ::std::option::Option<bool>,
        ///Instance URL is the URL of the instance.
        #[serde(
            rename = "instanceUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instance_url: ::std::option::Option<::std::string::String>,
        ///NeedsSetup is true when the instance has no users yet and requires
        /// initial setup (creating the first admin account). Unlike a
        /// null admin, this stays false once any user exists, so an
        /// instance that has lost its admins is not mistaken for a
        /// fresh install.
        #[serde(
            rename = "needsSetup",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub needs_setup: ::std::option::Option<bool>,
        ///Version is the current version of instance.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for InstanceProfile {
        fn default() -> Self {
            Self {
                admin: Default::default(),
                commit: Default::default(),
                demo: Default::default(),
                instance_url: Default::default(),
                needs_setup: Default::default(),
                version: Default::default(),
            }
        }
    }

    ///An instance setting resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An instance setting resource.",
    ///  "type": "object",
    ///  "properties": {
    ///    "aiSetting": {
    ///      "$ref": "#/components/schemas/InstanceSetting_AISetting"
    ///    },
    ///    "generalSetting": {
    ///      "$ref": "#/components/schemas/InstanceSetting_GeneralSetting"
    ///    },
    ///    "memoRelatedSetting": {
    ///      "$ref": "#/components/schemas/InstanceSetting_MemoRelatedSetting"
    ///    },
    ///    "name": {
    ///      "description": "The name of the instance setting.\n Format:
    /// instance/settings/{setting}",
    ///      "type": "string"
    ///    },
    ///    "notificationSetting": {
    ///      "$ref": "#/components/schemas/InstanceSetting_NotificationSetting"
    ///    },
    ///    "storageSetting": {
    ///      "$ref": "#/components/schemas/InstanceSetting_StorageSetting"
    ///    },
    ///    "tagsSetting": {
    ///      "$ref": "#/components/schemas/InstanceSetting_TagsSetting"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSetting {
        #[serde(
            rename = "aiSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ai_setting: ::std::option::Option<InstanceSettingAiSetting>,
        #[serde(
            rename = "generalSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub general_setting: ::std::option::Option<InstanceSettingGeneralSetting>,
        #[serde(
            rename = "memoRelatedSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_related_setting: ::std::option::Option<InstanceSettingMemoRelatedSetting>,
        ///The name of the instance setting.
        /// Format: instance/settings/{setting}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "notificationSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub notification_setting: ::std::option::Option<InstanceSettingNotificationSetting>,
        #[serde(
            rename = "storageSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub storage_setting: ::std::option::Option<InstanceSettingStorageSetting>,
        #[serde(
            rename = "tagsSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tags_setting: ::std::option::Option<InstanceSettingTagsSetting>,
    }

    impl ::std::default::Default for InstanceSetting {
        fn default() -> Self {
            Self {
                ai_setting: Default::default(),
                general_setting: Default::default(),
                memo_related_setting: Default::default(),
                name: Default::default(),
                notification_setting: Default::default(),
                storage_setting: Default::default(),
                tags_setting: Default::default(),
            }
        }
    }

    ///AIProviderConfig represents one callable AI provider connection.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AIProviderConfig represents one callable AI provider
    /// connection.",
    ///  "type": "object",
    ///  "properties": {
    ///    "apiKey": {
    ///      "description": "api_key is write-only and is never returned by
    /// GetInstanceSetting.",
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "apiKeyHint": {
    ///      "description": "api_key_hint is a masked hint for the stored API
    /// key.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "apiKeySet": {
    ///      "description": "api_key_set indicates whether an API key is stored
    /// for this provider.",
    ///      "readOnly": true,
    ///      "type": "boolean"
    ///    },
    ///    "endpoint": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "AI_PROVIDER_TYPE_UNSPECIFIED",
    ///        "OPENAI",
    ///        "GEMINI"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingAiProviderConfig {
        ///api_key is write-only and is never returned by GetInstanceSetting.
        #[serde(
            rename = "apiKey",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub api_key: ::std::option::Option<::std::string::String>,
        ///api_key_hint is a masked hint for the stored API key.
        #[serde(
            rename = "apiKeyHint",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub api_key_hint: ::std::option::Option<::std::string::String>,
        ///api_key_set indicates whether an API key is stored for this
        /// provider.
        #[serde(
            rename = "apiKeySet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub api_key_set: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub endpoint: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<InstanceSettingAiProviderConfigType>,
    }

    impl ::std::default::Default for InstanceSettingAiProviderConfig {
        fn default() -> Self {
            Self {
                api_key: Default::default(),
                api_key_hint: Default::default(),
                api_key_set: Default::default(),
                endpoint: Default::default(),
                id: Default::default(),
                title: Default::default(),
                type_: Default::default(),
            }
        }
    }

    ///`InstanceSettingAiProviderConfigType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "AI_PROVIDER_TYPE_UNSPECIFIED",
    ///    "OPENAI",
    ///    "GEMINI"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstanceSettingAiProviderConfigType {
        #[serde(rename = "AI_PROVIDER_TYPE_UNSPECIFIED")]
        AiProviderTypeUnspecified,
        #[serde(rename = "OPENAI")]
        Openai,
        #[serde(rename = "GEMINI")]
        Gemini,
    }

    impl ::std::fmt::Display for InstanceSettingAiProviderConfigType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AiProviderTypeUnspecified => f.write_str("AI_PROVIDER_TYPE_UNSPECIFIED"),
                Self::Openai => f.write_str("OPENAI"),
                Self::Gemini => f.write_str("GEMINI"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceSettingAiProviderConfigType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "AI_PROVIDER_TYPE_UNSPECIFIED" => Ok(Self::AiProviderTypeUnspecified),
                "OPENAI" => Ok(Self::Openai),
                "GEMINI" => Ok(Self::Gemini),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceSettingAiProviderConfigType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceSettingAiProviderConfigType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceSettingAiProviderConfigType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///AI provider configuration settings.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AI provider configuration settings.",
    ///  "type": "object",
    ///  "properties": {
    ///    "providers": {
    ///      "description": "providers is the list of AI provider configurations
    /// available instance-wide.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceSetting_AIProviderConfig"
    ///      }
    ///    },
    ///    "transcription": {
    ///      "description": "transcription is the speech-to-text feature
    /// configuration.\n When unset or transcription.provider_id is empty,
    /// transcription is disabled.",
    ///      "allOf": [
    ///        {
    ///          "$ref":
    /// "#/components/schemas/InstanceSetting_TranscriptionConfig"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingAiSetting {
        ///providers is the list of AI provider configurations available
        /// instance-wide.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub providers: ::std::vec::Vec<InstanceSettingAiProviderConfig>,
        ///transcription is the speech-to-text feature configuration.
        /// When unset or transcription.provider_id is empty, transcription is
        /// disabled.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub transcription: ::std::option::Option<InstanceSettingTranscriptionConfig>,
    }

    impl ::std::default::Default for InstanceSettingAiSetting {
        fn default() -> Self {
            Self {
                providers: Default::default(),
                transcription: Default::default(),
            }
        }
    }

    ///General instance settings configuration.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "General instance settings configuration.",
    ///  "type": "object",
    ///  "properties": {
    ///    "additionalScript": {
    ///      "description": "additional_script is the additional script.",
    ///      "type": "string"
    ///    },
    ///    "additionalStyle": {
    ///      "description": "additional_style is the additional style.",
    ///      "type": "string"
    ///    },
    ///    "customProfile": {
    ///      "description": "custom_profile is the custom profile.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GeneralSetting_CustomProfile"
    ///        }
    ///      ]
    ///    },
    ///    "disallowChangeNickname": {
    ///      "description": "disallow_change_nickname disallows changing
    /// nickname.",
    ///      "type": "boolean"
    ///    },
    ///    "disallowChangeUsername": {
    ///      "description": "disallow_change_username disallows changing
    /// username.",
    ///      "type": "boolean"
    ///    },
    ///    "disallowPasswordAuth": {
    ///      "description": "disallow_password_auth disallows password
    /// authentication.",
    ///      "type": "boolean"
    ///    },
    ///    "disallowUserRegistration": {
    ///      "description": "disallow_user_registration disallows user
    /// registration.",
    ///      "type": "boolean"
    ///    },
    ///    "weekStartDayOffset": {
    ///      "description": "week_start_day_offset is the week start day offset
    /// from Sunday.\n 0: Sunday, 1: Monday, 2: Tuesday, 3: Wednesday, 4:
    /// Thursday, 5: Friday, 6: Saturday\n Default is Sunday.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingGeneralSetting {
        ///additional_script is the additional script.
        #[serde(
            rename = "additionalScript",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub additional_script: ::std::option::Option<::std::string::String>,
        ///additional_style is the additional style.
        #[serde(
            rename = "additionalStyle",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub additional_style: ::std::option::Option<::std::string::String>,
        ///custom_profile is the custom profile.
        #[serde(
            rename = "customProfile",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub custom_profile: ::std::option::Option<GeneralSettingCustomProfile>,
        ///disallow_change_nickname disallows changing nickname.
        #[serde(
            rename = "disallowChangeNickname",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disallow_change_nickname: ::std::option::Option<bool>,
        ///disallow_change_username disallows changing username.
        #[serde(
            rename = "disallowChangeUsername",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disallow_change_username: ::std::option::Option<bool>,
        ///disallow_password_auth disallows password authentication.
        #[serde(
            rename = "disallowPasswordAuth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disallow_password_auth: ::std::option::Option<bool>,
        ///disallow_user_registration disallows user registration.
        #[serde(
            rename = "disallowUserRegistration",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disallow_user_registration: ::std::option::Option<bool>,
        ///week_start_day_offset is the week start day offset from Sunday.
        /// 0: Sunday, 1: Monday, 2: Tuesday, 3: Wednesday, 4: Thursday, 5:
        /// Friday, 6: Saturday Default is Sunday.
        #[serde(
            rename = "weekStartDayOffset",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub week_start_day_offset: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for InstanceSettingGeneralSetting {
        fn default() -> Self {
            Self {
                additional_script: Default::default(),
                additional_style: Default::default(),
                custom_profile: Default::default(),
                disallow_change_nickname: Default::default(),
                disallow_change_username: Default::default(),
                disallow_password_auth: Default::default(),
                disallow_user_registration: Default::default(),
                week_start_day_offset: Default::default(),
            }
        }
    }

    ///Memo-related instance settings and policies.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Memo-related instance settings and policies.",
    ///  "type": "object",
    ///  "properties": {
    ///    "contentLengthLimit": {
    ///      "description": "content_length_limit is the limit of content
    /// length. Unit is byte.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "enableDoubleClickEdit": {
    ///      "description": "enable_double_click_edit enables editing on double
    /// click.",
    ///      "type": "boolean"
    ///    },
    ///    "reactions": {
    ///      "description": "reactions is the list of reactions.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingMemoRelatedSetting {
        ///content_length_limit is the limit of content length. Unit is byte.
        #[serde(
            rename = "contentLengthLimit",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub content_length_limit: ::std::option::Option<i32>,
        ///enable_double_click_edit enables editing on double click.
        #[serde(
            rename = "enableDoubleClickEdit",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub enable_double_click_edit: ::std::option::Option<bool>,
        ///reactions is the list of reactions.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub reactions: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::default::Default for InstanceSettingMemoRelatedSetting {
        fn default() -> Self {
            Self {
                content_length_limit: Default::default(),
                enable_double_click_edit: Default::default(),
                reactions: Default::default(),
            }
        }
    }

    ///Notification transport configuration.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Notification transport configuration.",
    ///  "type": "object",
    ///  "properties": {
    ///    "email": {
    ///      "$ref": "#/components/schemas/NotificationSetting_EmailSetting"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingNotificationSetting {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<NotificationSettingEmailSetting>,
    }

    impl ::std::default::Default for InstanceSettingNotificationSetting {
        fn default() -> Self {
            Self {
                email: Default::default(),
            }
        }
    }

    ///Storage is a configured attachment storage instance.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Storage is a configured attachment storage instance.",
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "description": "id is the stable identifier referenced by
    /// attachments.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "name is the human-readable storage name.",
    ///      "type": "string"
    ///    },
    ///    "s3Config": {
    ///      "$ref": "#/components/schemas/Storage_S3Config"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "STORAGE_TYPE_UNSPECIFIED",
    ///        "DATABASE",
    ///        "LOCAL",
    ///        "S3"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingStorage {
        ///id is the stable identifier referenced by attachments.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///name is the human-readable storage name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "s3Config",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub s3_config: ::std::option::Option<StorageS3Config>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<InstanceSettingStorageType>,
    }

    impl ::std::default::Default for InstanceSettingStorage {
        fn default() -> Self {
            Self {
                id: Default::default(),
                name: Default::default(),
                s3_config: Default::default(),
                type_: Default::default(),
            }
        }
    }

    ///Storage configuration settings for instance attachments.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Storage configuration settings for instance
    /// attachments.",
    ///  "type": "object",
    ///  "properties": {
    ///    "defaultStorageId": {
    ///      "description": "Storage used for new attachments.",
    ///      "type": "string"
    ///    },
    ///    "filepathTemplate": {
    ///      "description": "The template of file path.\n e.g.
    /// assets/{timestamp}_{filename}",
    ///      "type": "string"
    ///    },
    ///    "s3Config": {
    ///      "description": "Legacy compatibility field. New clients use
    /// storages.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/StorageSetting_S3Config"
    ///        }
    ///      ]
    ///    },
    ///    "storageType": {
    ///      "description": "Legacy compatibility field. New clients use
    /// default_storage_id.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "STORAGE_TYPE_UNSPECIFIED",
    ///        "DATABASE",
    ///        "LOCAL",
    ///        "S3"
    ///      ]
    ///    },
    ///    "storages": {
    ///      "description": "Configured storage instances, including inactive
    /// instances referenced by attachments.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceSetting_Storage"
    ///      }
    ///    },
    ///    "uploadSizeLimitMb": {
    ///      "description": "The max upload size in megabytes.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingStorageSetting {
        ///Storage used for new attachments.
        #[serde(
            rename = "defaultStorageId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub default_storage_id: ::std::option::Option<::std::string::String>,
        ///The template of file path.
        /// e.g. assets/{timestamp}_{filename}
        #[serde(
            rename = "filepathTemplate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub filepath_template: ::std::option::Option<::std::string::String>,
        ///Legacy compatibility field. New clients use storages.
        #[serde(
            rename = "s3Config",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub s3_config: ::std::option::Option<StorageSettingS3Config>,
        ///Legacy compatibility field. New clients use default_storage_id.
        #[serde(
            rename = "storageType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub storage_type: ::std::option::Option<InstanceSettingStorageSettingStorageType>,
        ///Configured storage instances, including inactive instances
        /// referenced by attachments.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub storages: ::std::vec::Vec<InstanceSettingStorage>,
        ///The max upload size in megabytes.
        #[serde(
            rename = "uploadSizeLimitMb",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub upload_size_limit_mb: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for InstanceSettingStorageSetting {
        fn default() -> Self {
            Self {
                default_storage_id: Default::default(),
                filepath_template: Default::default(),
                s3_config: Default::default(),
                storage_type: Default::default(),
                storages: Default::default(),
                upload_size_limit_mb: Default::default(),
            }
        }
    }

    ///Legacy compatibility field. New clients use default_storage_id.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Legacy compatibility field. New clients use
    /// default_storage_id.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STORAGE_TYPE_UNSPECIFIED",
    ///    "DATABASE",
    ///    "LOCAL",
    ///    "S3"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstanceSettingStorageSettingStorageType {
        #[serde(rename = "STORAGE_TYPE_UNSPECIFIED")]
        StorageTypeUnspecified,
        #[serde(rename = "DATABASE")]
        Database,
        #[serde(rename = "LOCAL")]
        Local,
        S3,
    }

    impl ::std::fmt::Display for InstanceSettingStorageSettingStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StorageTypeUnspecified => f.write_str("STORAGE_TYPE_UNSPECIFIED"),
                Self::Database => f.write_str("DATABASE"),
                Self::Local => f.write_str("LOCAL"),
                Self::S3 => f.write_str("S3"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceSettingStorageSettingStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STORAGE_TYPE_UNSPECIFIED" => Ok(Self::StorageTypeUnspecified),
                "DATABASE" => Ok(Self::Database),
                "LOCAL" => Ok(Self::Local),
                "S3" => Ok(Self::S3),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceSettingStorageSettingStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceSettingStorageSettingStorageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceSettingStorageSettingStorageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`InstanceSettingStorageType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STORAGE_TYPE_UNSPECIFIED",
    ///    "DATABASE",
    ///    "LOCAL",
    ///    "S3"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstanceSettingStorageType {
        #[serde(rename = "STORAGE_TYPE_UNSPECIFIED")]
        StorageTypeUnspecified,
        #[serde(rename = "DATABASE")]
        Database,
        #[serde(rename = "LOCAL")]
        Local,
        S3,
    }

    impl ::std::fmt::Display for InstanceSettingStorageType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StorageTypeUnspecified => f.write_str("STORAGE_TYPE_UNSPECIFIED"),
                Self::Database => f.write_str("DATABASE"),
                Self::Local => f.write_str("LOCAL"),
                Self::S3 => f.write_str("S3"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceSettingStorageType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STORAGE_TYPE_UNSPECIFIED" => Ok(Self::StorageTypeUnspecified),
                "DATABASE" => Ok(Self::Database),
                "LOCAL" => Ok(Self::Local),
                "S3" => Ok(Self::S3),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceSettingStorageType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceSettingStorageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceSettingStorageType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Metadata for a tag.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Metadata for a tag.",
    ///  "type": "object",
    ///  "properties": {
    ///    "backgroundColor": {
    ///      "description": "Optional background color for the tag label.\n When
    /// unset, the default tag color is used.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Color"
    ///        }
    ///      ]
    ///    },
    ///    "blurContent": {
    ///      "description": "Whether memos with this tag should have their
    /// content blurred.",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingTagMetadata {
        ///Optional background color for the tag label.
        /// When unset, the default tag color is used.
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<Color>,
        ///Whether memos with this tag should have their content blurred.
        #[serde(
            rename = "blurContent",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub blur_content: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for InstanceSettingTagMetadata {
        fn default() -> Self {
            Self {
                background_color: Default::default(),
                blur_content: Default::default(),
            }
        }
    }

    ///Tag metadata configuration.
    /// Active tag metadata is stored in per-user tag settings.
    /// This message remains for backward compatibility with existing clients
    /// and migrations.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Tag metadata configuration.\n Active tag metadata is
    /// stored in per-user tag settings.\n This message remains for backward
    /// compatibility with existing clients and migrations.",
    ///  "type": "object",
    ///  "properties": {
    ///    "tags": {
    ///      "description": "Map of tag name pattern to tag metadata.\n Each key is treated as an anchored regular expression (^pattern$),\n so a single entry like \"project/.*\" matches all tags under that prefix.\n Exact tag names are also valid (they are trivially valid regex patterns).",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/InstanceSetting_TagMetadata"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingTagsSetting {
        ///Map of tag name pattern to tag metadata.
        /// Each key is treated as an anchored regular expression (^pattern$),
        /// so a single entry like "project/.*" matches all tags under that
        /// prefix. Exact tag names are also valid (they are trivially
        /// valid regex patterns).
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub tags: ::std::collections::HashMap<::std::string::String, InstanceSettingTagMetadata>,
    }

    impl ::std::default::Default for InstanceSettingTagsSetting {
        fn default() -> Self {
            Self {
                tags: Default::default(),
            }
        }
    }

    ///TranscriptionConfig configures the speech-to-text feature.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "TranscriptionConfig configures the speech-to-text
    /// feature.",
    ///  "type": "object",
    ///  "properties": {
    ///    "language": {
    ///      "description": "language is the default ISO 639-1 language hint sent to the provider.\n Empty string lets the provider auto-detect.",
    ///      "type": "string"
    ///    },
    ///    "model": {
    ///      "description": "model is the provider-specific model identifier.\n
    /// Empty string falls back to the engine default\n (whisper-1 for OPENAI
    /// providers, gemini-2.5-flash for GEMINI providers).",
    ///      "type": "string"
    ///    },
    ///    "prompt": {
    ///      "description": "prompt is a default spelling/vocabulary hint passed
    /// to the provider.",
    ///      "type": "string"
    ///    },
    ///    "providerId": {
    ///      "description": "provider_id references an entry in
    /// AISetting.providers[].id.\n Empty string means transcription is
    /// disabled.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSettingTranscriptionConfig {
        ///language is the default ISO 639-1 language hint sent to the
        /// provider. Empty string lets the provider auto-detect.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        ///model is the provider-specific model identifier.
        /// Empty string falls back to the engine default
        /// (whisper-1 for OPENAI providers, gemini-2.5-flash for GEMINI
        /// providers).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model: ::std::option::Option<::std::string::String>,
        ///prompt is a default spelling/vocabulary hint passed to the provider.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prompt: ::std::option::Option<::std::string::String>,
        ///provider_id references an entry in AISetting.providers[].id.
        /// Empty string means transcription is disabled.
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for InstanceSettingTranscriptionConfig {
        fn default() -> Self {
            Self {
                language: Default::default(),
                model: Default::default(),
                prompt: Default::default(),
                provider_id: Default::default(),
            }
        }
    }

    ///Resource usage statistics for the instance.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Resource usage statistics for the instance.",
    ///  "type": "object",
    ///  "properties": {
    ///    "database": {
    ///      "$ref": "#/components/schemas/InstanceStats_DatabaseStats"
    ///    },
    ///    "generatedTime": {
    ///      "description": "Server-side timestamp when the snapshot was
    /// generated.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "localStorageBytes": {
    ///      "description": "Recursive size of the data directory in bytes. -1
    /// if unavailable.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceStats {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub database: ::std::option::Option<InstanceStatsDatabaseStats>,
        ///Server-side timestamp when the snapshot was generated.
        #[serde(
            rename = "generatedTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generated_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Recursive size of the data directory in bytes. -1 if unavailable.
        #[serde(
            rename = "localStorageBytes",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub local_storage_bytes: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for InstanceStats {
        fn default() -> Self {
            Self {
                database: Default::default(),
                generated_time: Default::default(),
                local_storage_bytes: Default::default(),
            }
        }
    }

    ///Database size statistics.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Database size statistics.",
    ///  "type": "object",
    ///  "properties": {
    ///    "driver": {
    ///      "description": "driver is one of \"sqlite\", \"mysql\",
    /// \"postgres\".",
    ///      "type": "string"
    ///    },
    ///    "sizeBytes": {
    ///      "description": "size_bytes is the database size in bytes; -1 if
    /// unavailable.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceStatsDatabaseStats {
        ///driver is one of "sqlite", "mysql", "postgres".
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub driver: ::std::option::Option<::std::string::String>,
        ///size_bytes is the database size in bytes; -1 if unavailable.
        #[serde(
            rename = "sizeBytes",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub size_bytes: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for InstanceStatsDatabaseStats {
        fn default() -> Self {
            Self {
                driver: Default::default(),
                size_bytes: Default::default(),
            }
        }
    }

    ///`LinkMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "description": "The link description.",
    ///      "type": "string"
    ///    },
    ///    "image": {
    ///      "description": "The link image URL.",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "The link title.",
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "description": "The original link URL.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LinkMetadata {
        ///The link description.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///The link image URL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub image: ::std::option::Option<::std::string::String>,
        ///The link title.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        ///The original link URL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for LinkMetadata {
        fn default() -> Self {
            Self {
                description: Default::default(),
                image: Default::default(),
                title: Default::default(),
                url: Default::default(),
            }
        }
    }

    ///LinkedIdentity represents an SSO identity linked to a user account.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "LinkedIdentity represents an SSO identity linked to a
    /// user account.",
    ///  "type": "object",
    ///  "properties": {
    ///    "externUid": {
    ///      "description": "The external user identifier from the identity
    /// provider.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "idpName": {
    ///      "description": "The resource name of the identity provider.\n
    /// Format: identity-providers/{idp}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the linked identity.\n Format:
    /// users/{user}/linkedIdentities/{linked_identity}",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LinkedIdentity {
        ///The external user identifier from the identity provider.
        #[serde(
            rename = "externUid",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub extern_uid: ::std::option::Option<::std::string::String>,
        ///The resource name of the identity provider.
        /// Format: identity-providers/{idp}
        #[serde(
            rename = "idpName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub idp_name: ::std::option::Option<::std::string::String>,
        ///The resource name of the linked identity.
        /// Format: users/{user}/linkedIdentities/{linked_identity}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for LinkedIdentity {
        fn default() -> Self {
            Self {
                extern_uid: Default::default(),
                idp_name: Default::default(),
                name: Default::default(),
            }
        }
    }

    ///`ListAllUserStatsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "stats": {
    ///      "description": "The list of user statistics.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserStats"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListAllUserStatsResponse {
        ///The list of user statistics.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub stats: ::std::vec::Vec<UserStats>,
    }

    impl ::std::default::Default for ListAllUserStatsResponse {
        fn default() -> Self {
            Self {
                stats: Default::default(),
            }
        }
    }

    ///`ListAttachmentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attachments": {
    ///      "description": "The list of attachments.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Attachment"
    ///      }
    ///    },
    ///    "nextPageToken": {
    ///      "description": "A token that can be sent as `page_token` to
    /// retrieve the next page.\n If this field is omitted, there are no
    /// subsequent pages.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListAttachmentsResponse {
        ///The list of attachments.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub attachments: ::std::vec::Vec<Attachment>,
        ///A token that can be sent as `page_token` to retrieve the next page.
        /// If this field is omitted, there are no subsequent pages.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ListAttachmentsResponse {
        fn default() -> Self {
            Self {
                attachments: Default::default(),
                next_page_token: Default::default(),
            }
        }
    }

    ///`ListIdentityProvidersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "identityProviders": {
    ///      "description": "The list of identity providers.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IdentityProvider"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListIdentityProvidersResponse {
        ///The list of identity providers.
        #[serde(
            rename = "identityProviders",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub identity_providers: ::std::vec::Vec<IdentityProvider>,
    }

    impl ::std::default::Default for ListIdentityProvidersResponse {
        fn default() -> Self {
            Self {
                identity_providers: Default::default(),
            }
        }
    }

    ///`ListLinkedIdentitiesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "linkedIdentities": {
    ///      "description": "The list of linked identities.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/LinkedIdentity"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListLinkedIdentitiesResponse {
        ///The list of linked identities.
        #[serde(
            rename = "linkedIdentities",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub linked_identities: ::std::vec::Vec<LinkedIdentity>,
    }

    impl ::std::default::Default for ListLinkedIdentitiesResponse {
        fn default() -> Self {
            Self {
                linked_identities: Default::default(),
            }
        }
    }

    ///`ListMemoAttachmentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attachments": {
    ///      "description": "The list of attachments.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Attachment"
    ///      }
    ///    },
    ///    "nextPageToken": {
    ///      "description": "A token for the next page of results.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemoAttachmentsResponse {
        ///The list of attachments.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub attachments: ::std::vec::Vec<Attachment>,
        ///A token for the next page of results.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ListMemoAttachmentsResponse {
        fn default() -> Self {
            Self {
                attachments: Default::default(),
                next_page_token: Default::default(),
            }
        }
    }

    ///`ListMemoCommentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memos": {
    ///      "description": "The list of comment memos.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Memo"
    ///      }
    ///    },
    ///    "nextPageToken": {
    ///      "description": "A token for the next page of results.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemoCommentsResponse {
        ///The list of comment memos.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub memos: ::std::vec::Vec<Memo>,
        ///A token for the next page of results.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ListMemoCommentsResponse {
        fn default() -> Self {
            Self {
                memos: Default::default(),
                next_page_token: Default::default(),
            }
        }
    }

    ///`ListMemoReactionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextPageToken": {
    ///      "description": "A token for the next page of results.",
    ///      "type": "string"
    ///    },
    ///    "reactions": {
    ///      "description": "The list of reactions.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Reaction"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemoReactionsResponse {
        ///A token for the next page of results.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        ///The list of reactions.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub reactions: ::std::vec::Vec<Reaction>,
    }

    impl ::std::default::Default for ListMemoReactionsResponse {
        fn default() -> Self {
            Self {
                next_page_token: Default::default(),
                reactions: Default::default(),
            }
        }
    }

    ///`ListMemoRelationsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextPageToken": {
    ///      "description": "A token for the next page of results.",
    ///      "type": "string"
    ///    },
    ///    "relations": {
    ///      "description": "The list of relations.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MemoRelation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemoRelationsResponse {
        ///A token for the next page of results.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        ///The list of relations.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub relations: ::std::vec::Vec<MemoRelation>,
    }

    impl ::std::default::Default for ListMemoRelationsResponse {
        fn default() -> Self {
            Self {
                next_page_token: Default::default(),
                relations: Default::default(),
            }
        }
    }

    ///`ListMemoSharesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memoShares": {
    ///      "description": "The list of share links.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MemoShare"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemoSharesResponse {
        ///The list of share links.
        #[serde(
            rename = "memoShares",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub memo_shares: ::std::vec::Vec<MemoShare>,
    }

    impl ::std::default::Default for ListMemoSharesResponse {
        fn default() -> Self {
            Self {
                memo_shares: Default::default(),
            }
        }
    }

    ///`ListMemoViewsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memoViews": {
    ///      "description": "The list of memo views.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MemoView"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemoViewsResponse {
        ///The list of memo views.
        #[serde(
            rename = "memoViews",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub memo_views: ::std::vec::Vec<MemoView>,
    }

    impl ::std::default::Default for ListMemoViewsResponse {
        fn default() -> Self {
            Self {
                memo_views: Default::default(),
            }
        }
    }

    ///`ListMemosResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memos": {
    ///      "description": "The list of memos.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Memo"
    ///      }
    ///    },
    ///    "nextPageToken": {
    ///      "description": "A token that can be sent as `page_token` to
    /// retrieve the next page.\n If this field is omitted, there are no
    /// subsequent pages.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListMemosResponse {
        ///The list of memos.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub memos: ::std::vec::Vec<Memo>,
        ///A token that can be sent as `page_token` to retrieve the next page.
        /// If this field is omitted, there are no subsequent pages.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ListMemosResponse {
        fn default() -> Self {
            Self {
                memos: Default::default(),
                next_page_token: Default::default(),
            }
        }
    }

    ///`ListPersonalAccessTokensResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextPageToken": {
    ///      "description": "A token for the next page of results.",
    ///      "type": "string"
    ///    },
    ///    "personalAccessTokens": {
    ///      "description": "The list of personal access tokens.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PersonalAccessToken"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListPersonalAccessTokensResponse {
        ///A token for the next page of results.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        ///The list of personal access tokens.
        #[serde(
            rename = "personalAccessTokens",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub personal_access_tokens: ::std::vec::Vec<PersonalAccessToken>,
    }

    impl ::std::default::Default for ListPersonalAccessTokensResponse {
        fn default() -> Self {
            Self {
                next_page_token: Default::default(),
                personal_access_tokens: Default::default(),
            }
        }
    }

    ///`ListUserNotificationsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextPageToken": {
    ///      "type": "string"
    ///    },
    ///    "notifications": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserNotification"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListUserNotificationsResponse {
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub notifications: ::std::vec::Vec<UserNotification>,
    }

    impl ::std::default::Default for ListUserNotificationsResponse {
        fn default() -> Self {
            Self {
                next_page_token: Default::default(),
                notifications: Default::default(),
            }
        }
    }

    ///Response message for ListUserSettings method.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Response message for ListUserSettings method.",
    ///  "type": "object",
    ///  "properties": {
    ///    "nextPageToken": {
    ///      "description": "A token that can be sent as `page_token` to
    /// retrieve the next page.\n If this field is omitted, there are no
    /// subsequent pages.",
    ///      "type": "string"
    ///    },
    ///    "settings": {
    ///      "description": "The list of user settings.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserSetting"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListUserSettingsResponse {
        ///A token that can be sent as `page_token` to retrieve the next page.
        /// If this field is omitted, there are no subsequent pages.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        ///The list of user settings.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub settings: ::std::vec::Vec<UserSetting>,
    }

    impl ::std::default::Default for ListUserSettingsResponse {
        fn default() -> Self {
            Self {
                next_page_token: Default::default(),
                settings: Default::default(),
            }
        }
    }

    ///`ListUserWebhooksResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "webhooks": {
    ///      "description": "The list of webhooks.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserWebhook"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListUserWebhooksResponse {
        ///The list of webhooks.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub webhooks: ::std::vec::Vec<UserWebhook>,
    }

    impl ::std::default::Default for ListUserWebhooksResponse {
        fn default() -> Self {
            Self {
                webhooks: Default::default(),
            }
        }
    }

    ///`ListUsersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextPageToken": {
    ///      "description": "A token that can be sent as `page_token` to
    /// retrieve the next page.\n If this field is omitted, there are no
    /// subsequent pages.",
    ///      "type": "string"
    ///    },
    ///    "users": {
    ///      "description": "The list of users.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/User"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ListUsersResponse {
        ///A token that can be sent as `page_token` to retrieve the next page.
        /// If this field is omitted, there are no subsequent pages.
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        ///The list of users.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub users: ::std::vec::Vec<User>,
    }

    impl ::std::default::Default for ListUsersResponse {
        fn default() -> Self {
            Self {
                next_page_token: Default::default(),
                users: Default::default(),
            }
        }
    }

    ///`Location`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "latitude": {
    ///      "description": "The latitude of the location.",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longitude": {
    ///      "description": "The longitude of the location.",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "placeholder": {
    ///      "description": "A placeholder text for the location.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Location {
        ///The latitude of the location.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<f64>,
        ///The longitude of the location.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<f64>,
        ///A placeholder text for the location.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub placeholder: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Location {
        fn default() -> Self {
            Self {
                latitude: Default::default(),
                longitude: Default::default(),
                placeholder: Default::default(),
            }
        }
    }

    ///`MediaCaptureTime`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "localDateTime": {
    ///      "type": "string"
    ///    },
    ///    "utcOffset": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MediaCaptureTime {
        #[serde(
            rename = "localDateTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub local_date_time: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "utcOffset",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub utc_offset: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for MediaCaptureTime {
        fn default() -> Self {
            Self {
                local_date_time: Default::default(),
                utc_offset: Default::default(),
            }
        }
    }

    ///`MediaLocation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "altitudeMeters": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "latitude": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longitude": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MediaLocation {
        #[serde(
            rename = "altitudeMeters",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub altitude_meters: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for MediaLocation {
        fn default() -> Self {
            Self {
                altitude_meters: Default::default(),
                latitude: Default::default(),
                longitude: Default::default(),
            }
        }
    }

    ///`MediaMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "height": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "photo": {
    ///      "$ref": "#/components/schemas/PhotoMetadata"
    ///    },
    ///    "video": {
    ///      "$ref": "#/components/schemas/VideoMetadata"
    ///    },
    ///    "width": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MediaMetadata {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub height: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub photo: ::std::option::Option<PhotoMetadata>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub video: ::std::option::Option<VideoMetadata>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub width: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for MediaMetadata {
        fn default() -> Self {
            Self {
                height: Default::default(),
                photo: Default::default(),
                video: Default::default(),
                width: Default::default(),
            }
        }
    }

    ///`Memo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "content",
    ///    "state",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "attachments": {
    ///      "description": "Optional. The attachments of the memo.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Attachment"
    ///      }
    ///    },
    ///    "content": {
    ///      "description": "Required. The content of the memo in Markdown
    /// format.",
    ///      "type": "string"
    ///    },
    ///    "createTime": {
    ///      "description": "The creation timestamp.\n If not set on creation,
    /// the server will set it to the current time.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "creator": {
    ///      "description": "The name of the creator.\n Format: users/{user}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "location": {
    ///      "description": "Optional. The location of the memo.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Location"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the memo.\n Format:
    /// memos/{memo}, where memo is the user-defined UID.",
    ///      "type": "string"
    ///    },
    ///    "parent": {
    ///      "description": "Output only. The name of the parent memo.\n Format:
    /// memos/{memo}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "pinned": {
    ///      "description": "Whether the memo is pinned.",
    ///      "type": "boolean"
    ///    },
    ///    "property": {
    ///      "description": "Output only. The computed properties of the memo.",
    ///      "readOnly": true,
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Memo_Property"
    ///        }
    ///      ]
    ///    },
    ///    "reactions": {
    ///      "description": "Output only. The reactions to the memo.",
    ///      "readOnly": true,
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Reaction"
    ///      }
    ///    },
    ///    "relations": {
    ///      "description": "Optional. The relations of the memo.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MemoRelation"
    ///      }
    ///    },
    ///    "snippet": {
    ///      "description": "Output only. The snippet of the memo content. Plain
    /// text only.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "description": "The state of the memo.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "STATE_UNSPECIFIED",
    ///        "NORMAL",
    ///        "ARCHIVED"
    ///      ]
    ///    },
    ///    "tags": {
    ///      "description": "Output only. The tags extracted from the content.",
    ///      "readOnly": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "updateTime": {
    ///      "description": "The last update timestamp.\n If not set on
    /// creation, the server will set it to the current time.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "visibility": {
    ///      "description": "The visibility of the memo.\n One of PRIVATE
    /// (creator only), PROTECTED (signed-in users), or\n PUBLIC (anyone).
    /// Defaults to PRIVATE on creation when unspecified.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "VISIBILITY_UNSPECIFIED",
    ///        "PRIVATE",
    ///        "PROTECTED",
    ///        "PUBLIC"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Memo {
        ///Optional. The attachments of the memo.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub attachments: ::std::vec::Vec<Attachment>,
        ///Required. The content of the memo in Markdown format.
        pub content: ::std::string::String,
        ///The creation timestamp.
        /// If not set on creation, the server will set it to the current time.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The name of the creator.
        /// Format: users/{user}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub creator: ::std::option::Option<::std::string::String>,
        ///Optional. The location of the memo.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<Location>,
        ///The resource name of the memo.
        /// Format: memos/{memo}, where memo is the user-defined UID.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Output only. The name of the parent memo.
        /// Format: memos/{memo}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<::std::string::String>,
        ///Whether the memo is pinned.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pinned: ::std::option::Option<bool>,
        ///Output only. The computed properties of the memo.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub property: ::std::option::Option<MemoProperty>,
        ///Output only. The reactions to the memo.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub reactions: ::std::vec::Vec<Reaction>,
        ///Optional. The relations of the memo.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub relations: ::std::vec::Vec<MemoRelation>,
        ///Output only. The snippet of the memo content. Plain text only.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippet: ::std::option::Option<::std::string::String>,
        ///The state of the memo.
        pub state: MemoState,
        ///Output only. The tags extracted from the content.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        ///The last update timestamp.
        /// If not set on creation, the server will set it to the current time.
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The visibility of the memo.
        /// One of PRIVATE (creator only), PROTECTED (signed-in users), or
        /// PUBLIC (anyone). Defaults to PRIVATE on creation when unspecified.
        pub visibility: MemoVisibility,
    }

    ///Computed properties of a memo.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Computed properties of a memo.",
    ///  "type": "object",
    ///  "properties": {
    ///    "hasCode": {
    ///      "type": "boolean"
    ///    },
    ///    "hasIncompleteTasks": {
    ///      "type": "boolean"
    ///    },
    ///    "hasLink": {
    ///      "type": "boolean"
    ///    },
    ///    "hasTaskList": {
    ///      "type": "boolean"
    ///    },
    ///    "title": {
    ///      "description": "The title extracted from the first H1 heading, if
    /// present.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MemoProperty {
        #[serde(
            rename = "hasCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_code: ::std::option::Option<bool>,
        #[serde(
            rename = "hasIncompleteTasks",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_incomplete_tasks: ::std::option::Option<bool>,
        #[serde(
            rename = "hasLink",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_link: ::std::option::Option<bool>,
        #[serde(
            rename = "hasTaskList",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_task_list: ::std::option::Option<bool>,
        ///The title extracted from the first H1 heading, if present.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for MemoProperty {
        fn default() -> Self {
            Self {
                has_code: Default::default(),
                has_incomplete_tasks: Default::default(),
                has_link: Default::default(),
                has_task_list: Default::default(),
                title: Default::default(),
            }
        }
    }

    ///`MemoRelation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "memo",
    ///    "relatedMemo",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "memo": {
    ///      "description": "The memo in the relation.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MemoRelation_Memo"
    ///        }
    ///      ]
    ///    },
    ///    "relatedMemo": {
    ///      "description": "The related memo.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MemoRelation_Memo"
    ///        }
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "TYPE_UNSPECIFIED",
    ///        "REFERENCE",
    ///        "COMMENT"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MemoRelation {
        ///The memo in the relation.
        pub memo: MemoRelationMemo,
        ///The related memo.
        #[serde(rename = "relatedMemo")]
        pub related_memo: MemoRelationMemo,
        #[serde(rename = "type")]
        pub type_: MemoRelationType,
    }

    ///Memo reference in relations.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Memo reference in relations.",
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "The resource name of the memo.\n Format:
    /// memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "snippet": {
    ///      "description": "Output only. The snippet of the memo content. Plain
    /// text only.",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MemoRelationMemo {
        ///The resource name of the memo.
        /// Format: memos/{memo}
        pub name: ::std::string::String,
        ///Output only. The snippet of the memo content. Plain text only.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippet: ::std::option::Option<::std::string::String>,
    }

    ///`MemoRelationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "TYPE_UNSPECIFIED",
    ///    "REFERENCE",
    ///    "COMMENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MemoRelationType {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        TypeUnspecified,
        #[serde(rename = "REFERENCE")]
        Reference,
        #[serde(rename = "COMMENT")]
        Comment,
    }

    impl ::std::fmt::Display for MemoRelationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::TypeUnspecified => f.write_str("TYPE_UNSPECIFIED"),
                Self::Reference => f.write_str("REFERENCE"),
                Self::Comment => f.write_str("COMMENT"),
            }
        }
    }

    impl ::std::str::FromStr for MemoRelationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TYPE_UNSPECIFIED" => Ok(Self::TypeUnspecified),
                "REFERENCE" => Ok(Self::Reference),
                "COMMENT" => Ok(Self::Comment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MemoRelationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MemoRelationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MemoRelationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MemoServiceListMemosState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STATE_UNSPECIFIED",
    ///    "NORMAL",
    ///    "ARCHIVED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MemoServiceListMemosState {
        #[serde(rename = "STATE_UNSPECIFIED")]
        StateUnspecified,
        #[serde(rename = "NORMAL")]
        Normal,
        #[serde(rename = "ARCHIVED")]
        Archived,
    }

    impl ::std::fmt::Display for MemoServiceListMemosState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StateUnspecified => f.write_str("STATE_UNSPECIFIED"),
                Self::Normal => f.write_str("NORMAL"),
                Self::Archived => f.write_str("ARCHIVED"),
            }
        }
    }

    impl ::std::str::FromStr for MemoServiceListMemosState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STATE_UNSPECIFIED" => Ok(Self::StateUnspecified),
                "NORMAL" => Ok(Self::Normal),
                "ARCHIVED" => Ok(Self::Archived),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MemoServiceListMemosState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MemoServiceListMemosState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MemoServiceListMemosState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MemoShare is an access grant that permits read-only access to a memo via
    /// an opaque bearer token.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "MemoShare is an access grant that permits read-only
    /// access to a memo via an opaque bearer token.",
    ///  "type": "object",
    ///  "properties": {
    ///    "createTime": {
    ///      "description": "Output only. When this share link was created.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "expireTime": {
    ///      "description": "Optional. When set, the share link stops working
    /// after this time.\n If unset, the link never expires.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the share. Format:
    /// memos/{memo}/shares/{share}\n The {share} segment is the opaque token
    /// used in the share URL.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MemoShare {
        ///Output only. When this share link was created.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Optional. When set, the share link stops working after this time.
        /// If unset, the link never expires.
        #[serde(
            rename = "expireTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expire_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The resource name of the share. Format: memos/{memo}/shares/{share}
        /// The {share} segment is the opaque token used in the share URL.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for MemoShare {
        fn default() -> Self {
            Self {
                create_time: Default::default(),
                expire_time: Default::default(),
                name: Default::default(),
            }
        }
    }

    ///The state of the memo.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The state of the memo.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STATE_UNSPECIFIED",
    ///    "NORMAL",
    ///    "ARCHIVED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MemoState {
        #[serde(rename = "STATE_UNSPECIFIED")]
        StateUnspecified,
        #[serde(rename = "NORMAL")]
        Normal,
        #[serde(rename = "ARCHIVED")]
        Archived,
    }

    impl ::std::fmt::Display for MemoState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StateUnspecified => f.write_str("STATE_UNSPECIFIED"),
                Self::Normal => f.write_str("NORMAL"),
                Self::Archived => f.write_str("ARCHIVED"),
            }
        }
    }

    impl ::std::str::FromStr for MemoState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STATE_UNSPECIFIED" => Ok(Self::StateUnspecified),
                "NORMAL" => Ok(Self::Normal),
                "ARCHIVED" => Ok(Self::Archived),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MemoState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MemoState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MemoState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MemoView`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "filter",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "filter": {
    ///      "description": "The CEL filter expression for the memo view, using
    /// the same grammar as the\n ListMemos `filter` argument. Reuse it by
    /// passing this value to ListMemos.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the memo view.\n Format:
    /// users/{user}/views/{view}",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "The title of the memo view.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MemoView {
        ///The CEL filter expression for the memo view, using the same grammar
        /// as the ListMemos `filter` argument. Reuse it by passing this
        /// value to ListMemos.
        pub filter: ::std::string::String,
        ///The resource name of the memo view.
        /// Format: users/{user}/views/{view}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The title of the memo view.
        pub title: ::std::string::String,
    }

    ///The visibility of the memo.
    /// One of PRIVATE (creator only), PROTECTED (signed-in users), or
    /// PUBLIC (anyone). Defaults to PRIVATE on creation when unspecified.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The visibility of the memo.\n One of PRIVATE (creator
    /// only), PROTECTED (signed-in users), or\n PUBLIC (anyone). Defaults to
    /// PRIVATE on creation when unspecified.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "VISIBILITY_UNSPECIFIED",
    ///    "PRIVATE",
    ///    "PROTECTED",
    ///    "PUBLIC"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MemoVisibility {
        #[serde(rename = "VISIBILITY_UNSPECIFIED")]
        VisibilityUnspecified,
        #[serde(rename = "PRIVATE")]
        Private,
        #[serde(rename = "PROTECTED")]
        Protected,
        #[serde(rename = "PUBLIC")]
        Public,
    }

    impl ::std::fmt::Display for MemoVisibility {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VisibilityUnspecified => f.write_str("VISIBILITY_UNSPECIFIED"),
                Self::Private => f.write_str("PRIVATE"),
                Self::Protected => f.write_str("PROTECTED"),
                Self::Public => f.write_str("PUBLIC"),
            }
        }
    }

    impl ::std::str::FromStr for MemoVisibility {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VISIBILITY_UNSPECIFIED" => Ok(Self::VisibilityUnspecified),
                "PRIVATE" => Ok(Self::Private),
                "PROTECTED" => Ok(Self::Protected),
                "PUBLIC" => Ok(Self::Public),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MemoVisibility {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MemoVisibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MemoVisibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MotionMedia`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "family": {
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "MOTION_MEDIA_FAMILY_UNSPECIFIED",
    ///        "APPLE_LIVE_PHOTO",
    ///        "ANDROID_MOTION_PHOTO"
    ///      ]
    ///    },
    ///    "groupId": {
    ///      "type": "string"
    ///    },
    ///    "hasEmbeddedVideo": {
    ///      "type": "boolean"
    ///    },
    ///    "presentationTimestampUs": {
    ///      "type": "string"
    ///    },
    ///    "role": {
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "MOTION_MEDIA_ROLE_UNSPECIFIED",
    ///        "STILL",
    ///        "VIDEO",
    ///        "CONTAINER"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MotionMedia {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub family: ::std::option::Option<MotionMediaFamily>,
        #[serde(
            rename = "groupId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "hasEmbeddedVideo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_embedded_video: ::std::option::Option<bool>,
        #[serde(
            rename = "presentationTimestampUs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub presentation_timestamp_us: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<MotionMediaRole>,
    }

    impl ::std::default::Default for MotionMedia {
        fn default() -> Self {
            Self {
                family: Default::default(),
                group_id: Default::default(),
                has_embedded_video: Default::default(),
                presentation_timestamp_us: Default::default(),
                role: Default::default(),
            }
        }
    }

    ///`MotionMediaFamily`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "MOTION_MEDIA_FAMILY_UNSPECIFIED",
    ///    "APPLE_LIVE_PHOTO",
    ///    "ANDROID_MOTION_PHOTO"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MotionMediaFamily {
        #[serde(rename = "MOTION_MEDIA_FAMILY_UNSPECIFIED")]
        MotionMediaFamilyUnspecified,
        #[serde(rename = "APPLE_LIVE_PHOTO")]
        AppleLivePhoto,
        #[serde(rename = "ANDROID_MOTION_PHOTO")]
        AndroidMotionPhoto,
    }

    impl ::std::fmt::Display for MotionMediaFamily {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::MotionMediaFamilyUnspecified => {
                    f.write_str("MOTION_MEDIA_FAMILY_UNSPECIFIED")
                }
                Self::AppleLivePhoto => f.write_str("APPLE_LIVE_PHOTO"),
                Self::AndroidMotionPhoto => f.write_str("ANDROID_MOTION_PHOTO"),
            }
        }
    }

    impl ::std::str::FromStr for MotionMediaFamily {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MOTION_MEDIA_FAMILY_UNSPECIFIED" => Ok(Self::MotionMediaFamilyUnspecified),
                "APPLE_LIVE_PHOTO" => Ok(Self::AppleLivePhoto),
                "ANDROID_MOTION_PHOTO" => Ok(Self::AndroidMotionPhoto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MotionMediaFamily {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MotionMediaFamily {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MotionMediaFamily {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MotionMediaRole`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "MOTION_MEDIA_ROLE_UNSPECIFIED",
    ///    "STILL",
    ///    "VIDEO",
    ///    "CONTAINER"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MotionMediaRole {
        #[serde(rename = "MOTION_MEDIA_ROLE_UNSPECIFIED")]
        MotionMediaRoleUnspecified,
        #[serde(rename = "STILL")]
        Still,
        #[serde(rename = "VIDEO")]
        Video,
        #[serde(rename = "CONTAINER")]
        Container,
    }

    impl ::std::fmt::Display for MotionMediaRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::MotionMediaRoleUnspecified => f.write_str("MOTION_MEDIA_ROLE_UNSPECIFIED"),
                Self::Still => f.write_str("STILL"),
                Self::Video => f.write_str("VIDEO"),
                Self::Container => f.write_str("CONTAINER"),
            }
        }
    }

    impl ::std::str::FromStr for MotionMediaRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MOTION_MEDIA_ROLE_UNSPECIFIED" => Ok(Self::MotionMediaRoleUnspecified),
                "STILL" => Ok(Self::Still),
                "VIDEO" => Ok(Self::Video),
                "CONTAINER" => Ok(Self::Container),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MotionMediaRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MotionMediaRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MotionMediaRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Email delivery configuration for notifications.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Email delivery configuration for notifications.",
    ///  "type": "object",
    ///  "properties": {
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "fromEmail": {
    ///      "type": "string"
    ///    },
    ///    "fromName": {
    ///      "type": "string"
    ///    },
    ///    "replyTo": {
    ///      "type": "string"
    ///    },
    ///    "smtpHost": {
    ///      "type": "string"
    ///    },
    ///    "smtpPassword": {
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "smtpPort": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "smtpUsername": {
    ///      "type": "string"
    ///    },
    ///    "useSsl": {
    ///      "type": "boolean"
    ///    },
    ///    "useTls": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NotificationSettingEmailSetting {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(
            rename = "fromEmail",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub from_email: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "fromName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub from_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "replyTo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub reply_to: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "smtpHost",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub smtp_host: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "smtpPassword",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub smtp_password: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "smtpPort",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub smtp_port: ::std::option::Option<i32>,
        #[serde(
            rename = "smtpUsername",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub smtp_username: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "useSsl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub use_ssl: ::std::option::Option<bool>,
        #[serde(
            rename = "useTls",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub use_tls: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for NotificationSettingEmailSetting {
        fn default() -> Self {
            Self {
                enabled: Default::default(),
                from_email: Default::default(),
                from_name: Default::default(),
                reply_to: Default::default(),
                smtp_host: Default::default(),
                smtp_password: Default::default(),
                smtp_port: Default::default(),
                smtp_username: Default::default(),
                use_ssl: Default::default(),
                use_tls: Default::default(),
            }
        }
    }

    ///`OAuth2Config`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "authUrl": {
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "clientSecret": {
    ///      "type": "string"
    ///    },
    ///    "fieldMapping": {
    ///      "$ref": "#/components/schemas/FieldMapping"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "tokenUrl": {
    ///      "type": "string"
    ///    },
    ///    "userInfoUrl": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OAuth2Config {
        #[serde(
            rename = "authUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auth_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "clientSecret",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_secret: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "fieldMapping",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub field_mapping: ::std::option::Option<FieldMapping>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub scopes: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "tokenUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub token_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userInfoUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_info_url: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for OAuth2Config {
        fn default() -> Self {
            Self {
                auth_url: Default::default(),
                client_id: Default::default(),
                client_secret: Default::default(),
                field_mapping: Default::default(),
                scopes: Default::default(),
                token_url: Default::default(),
                user_info_url: Default::default(),
            }
        }
    }

    ///PersonalAccessToken represents a long-lived token for API/script access.
    /// PATs are distinct from short-lived JWT access tokens used for session
    /// authentication.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "PersonalAccessToken represents a long-lived token for
    /// API/script access.\n PATs are distinct from short-lived JWT access
    /// tokens used for session authentication.",
    ///  "type": "object",
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "Output only. The creation timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "description": {
    ///      "description": "The description of the token.",
    ///      "type": "string"
    ///    },
    ///    "expiresAt": {
    ///      "description": "Optional. The expiration timestamp.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "lastUsedAt": {
    ///      "description": "Output only. The last used timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the personal access token.\n
    /// Format: users/{user}/personalAccessTokens/{personal_access_token}",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PersonalAccessToken {
        ///Output only. The creation timestamp.
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The description of the token.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Optional. The expiration timestamp.
        #[serde(
            rename = "expiresAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Output only. The last used timestamp.
        #[serde(
            rename = "lastUsedAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_used_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The resource name of the personal access token.
        /// Format: users/{user}/personalAccessTokens/{personal_access_token}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for PersonalAccessToken {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                description: Default::default(),
                expires_at: Default::default(),
                last_used_at: Default::default(),
                name: Default::default(),
            }
        }
    }

    ///`PhotoMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cameraMake": {
    ///      "type": "string"
    ///    },
    ///    "cameraModel": {
    ///      "type": "string"
    ///    },
    ///    "captureTime": {
    ///      "$ref": "#/components/schemas/MediaCaptureTime"
    ///    },
    ///    "exposureTimeSeconds": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "fNumber": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "focalLengthMm": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "iso": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lensModel": {
    ///      "type": "string"
    ///    },
    ///    "location": {
    ///      "$ref": "#/components/schemas/MediaLocation"
    ///    },
    ///    "sourceExifOrientation": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PhotoMetadata {
        #[serde(
            rename = "cameraMake",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub camera_make: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "cameraModel",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub camera_model: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "captureTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub capture_time: ::std::option::Option<MediaCaptureTime>,
        #[serde(
            rename = "exposureTimeSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exposure_time_seconds: ::std::option::Option<f64>,
        #[serde(
            rename = "fNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub f_number: ::std::option::Option<f64>,
        #[serde(
            rename = "focalLengthMm",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub focal_length_mm: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub iso: ::std::option::Option<i32>,
        #[serde(
            rename = "lensModel",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub lens_model: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<MediaLocation>,
        #[serde(
            rename = "sourceExifOrientation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub source_exif_orientation: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for PhotoMetadata {
        fn default() -> Self {
            Self {
                camera_make: Default::default(),
                camera_model: Default::default(),
                capture_time: Default::default(),
                exposure_time_seconds: Default::default(),
                f_number: Default::default(),
                focal_length_mm: Default::default(),
                iso: Default::default(),
                lens_model: Default::default(),
                location: Default::default(),
                source_exif_orientation: Default::default(),
            }
        }
    }

    ///`Reaction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "contentId",
    ///    "reactionType"
    ///  ],
    ///  "properties": {
    ///    "contentId": {
    ///      "description": "The resource name of the content.\n For memo
    /// reactions, this should be the memo's resource name.\n Format:
    /// memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "createTime": {
    ///      "description": "Output only. The creation timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "creator": {
    ///      "description": "The resource name of the creator.\n Format:
    /// users/{user}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the reaction.\n Format:
    /// memos/{memo}/reactions/{reaction}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "reactionType": {
    ///      "description": "Required. The type of reaction (e.g., \"👍\",
    /// \"❤️\", \"😄\").",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Reaction {
        ///The resource name of the content.
        /// For memo reactions, this should be the memo's resource name.
        /// Format: memos/{memo}
        #[serde(rename = "contentId")]
        pub content_id: ::std::string::String,
        ///Output only. The creation timestamp.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The resource name of the creator.
        /// Format: users/{user}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub creator: ::std::option::Option<::std::string::String>,
        ///The resource name of the reaction.
        /// Format: memos/{memo}/reactions/{reaction}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Required. The type of reaction (e.g., "👍", "❤️", "😄").
        #[serde(rename = "reactionType")]
        pub reaction_type: ::std::string::String,
    }

    ///`RefreshTokenRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RefreshTokenRequest(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for RefreshTokenRequest {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RefreshTokenRequest>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RefreshTokenRequest) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RefreshTokenRequest
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///`RefreshTokenResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accessToken": {
    ///      "description": "The new short-lived access token.",
    ///      "type": "string"
    ///    },
    ///    "expiresAt": {
    ///      "description": "When the access token expires.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RefreshTokenResponse {
        ///The new short-lived access token.
        #[serde(
            rename = "accessToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_token: ::std::option::Option<::std::string::String>,
        ///When the access token expires.
        #[serde(
            rename = "expiresAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }

    impl ::std::default::Default for RefreshTokenResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                expires_at: Default::default(),
            }
        }
    }

    ///`SetMemoAttachmentsRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "attachments",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "attachments": {
    ///      "description": "Required. The attachments to set for the memo.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Attachment"
    ///      }
    ///    },
    ///    "name": {
    ///      "description": "Required. The resource name of the memo.\n Format:
    /// memos/{memo}",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SetMemoAttachmentsRequest {
        ///Required. The attachments to set for the memo.
        pub attachments: ::std::vec::Vec<Attachment>,
        ///Required. The resource name of the memo.
        /// Format: memos/{memo}
        pub name: ::std::string::String,
    }

    ///`SetMemoRelationsRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "relations"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "Required. The resource name of the memo.\n Format:
    /// memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "relations": {
    ///      "description": "Required. The relations to set for the memo.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MemoRelation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SetMemoRelationsRequest {
        ///Required. The resource name of the memo.
        /// Format: memos/{memo}
        pub name: ::std::string::String,
        ///Required. The relations to set for the memo.
        pub relations: ::std::vec::Vec<MemoRelation>,
    }

    ///`SignInRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "passwordCredentials": {
    ///      "description": "Username and password authentication.",
    ///      "allOf": [
    ///        {
    ///          "$ref":
    /// "#/components/schemas/SignInRequest_PasswordCredentials"
    ///        }
    ///      ]
    ///    },
    ///    "ssoCredentials": {
    ///      "description": "SSO provider authentication.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SignInRequest_SSOCredentials"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SignInRequest {
        ///Username and password authentication.
        #[serde(
            rename = "passwordCredentials",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub password_credentials: ::std::option::Option<SignInRequestPasswordCredentials>,
        ///SSO provider authentication.
        #[serde(
            rename = "ssoCredentials",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sso_credentials: ::std::option::Option<SignInRequestSsoCredentials>,
    }

    impl ::std::default::Default for SignInRequest {
        fn default() -> Self {
            Self {
                password_credentials: Default::default(),
                sso_credentials: Default::default(),
            }
        }
    }

    ///Nested message for password-based authentication credentials.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Nested message for password-based authentication
    /// credentials.",
    ///  "type": "object",
    ///  "required": [
    ///    "password",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "password": {
    ///      "description": "The password to sign in with.",
    ///      "type": "string"
    ///    },
    ///    "username": {
    ///      "description": "The username to sign in with.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SignInRequestPasswordCredentials {
        ///The password to sign in with.
        pub password: ::std::string::String,
        ///The username to sign in with.
        pub username: ::std::string::String,
    }

    ///Nested message for SSO authentication credentials.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Nested message for SSO authentication credentials.",
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "idpName",
    ///    "redirectUri"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "description": "The authorization code from the SSO provider.",
    ///      "type": "string"
    ///    },
    ///    "codeVerifier": {
    ///      "description": "The PKCE code verifier for enhanced security (RFC
    /// 7636).\n Optional - enables PKCE flow protection against authorization
    /// code interception.",
    ///      "type": "string"
    ///    },
    ///    "idpName": {
    ///      "description": "The resource name of the SSO provider.\n Format:
    /// identity-providers/{idp}",
    ///      "type": "string"
    ///    },
    ///    "redirectUri": {
    ///      "description": "The redirect URI used in the SSO flow.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SignInRequestSsoCredentials {
        ///The authorization code from the SSO provider.
        pub code: ::std::string::String,
        ///The PKCE code verifier for enhanced security (RFC 7636).
        /// Optional - enables PKCE flow protection against authorization code
        /// interception.
        #[serde(
            rename = "codeVerifier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub code_verifier: ::std::option::Option<::std::string::String>,
        ///The resource name of the SSO provider.
        /// Format: identity-providers/{idp}
        #[serde(rename = "idpName")]
        pub idp_name: ::std::string::String,
        ///The redirect URI used in the SSO flow.
        #[serde(rename = "redirectUri")]
        pub redirect_uri: ::std::string::String,
    }

    ///`SignInResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accessToken": {
    ///      "description": "The short-lived access token for API requests.\n
    /// Store in memory only, not in localStorage.",
    ///      "type": "string"
    ///    },
    ///    "accessTokenExpiresAt": {
    ///      "description": "When the access token expires.\n Client should call
    /// RefreshToken before this time.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "user": {
    ///      "description": "The authenticated user's information.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/User"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SignInResponse {
        ///The short-lived access token for API requests.
        /// Store in memory only, not in localStorage.
        #[serde(
            rename = "accessToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_token: ::std::option::Option<::std::string::String>,
        ///When the access token expires.
        /// Client should call RefreshToken before this time.
        #[serde(
            rename = "accessTokenExpiresAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_token_expires_at:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The authenticated user's information.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<User>,
    }

    impl ::std::default::Default for SignInResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expires_at: Default::default(),
                user: Default::default(),
            }
        }
    }

    ///The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).",
    ///  "type": "object",
    ///  "properties": {
    ///    "code": {
    ///      "description": "The status code, which should be an enum value of
    /// [google.rpc.Code][google.rpc.Code].",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "details": {
    ///      "description": "A list of messages that carry the error details.
    /// There is a common set of message types for APIs to use.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GoogleProtobufAny"
    ///      }
    ///    },
    ///    "message": {
    ///      "description": "A developer-facing error message, which should be
    /// in English. Any user-facing error message should be localized and sent
    /// in the [google.rpc.Status.details][google.rpc.Status.details] field, or
    /// localized by the client.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Status {
        ///The status code, which should be an enum value of
        /// [google.rpc.Code][google.rpc.Code].
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<i32>,
        ///A list of messages that carry the error details.  There is a common
        /// set of message types for APIs to use.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub details: ::std::vec::Vec<GoogleProtobufAny>,
        ///A developer-facing error message, which should be in English. Any
        /// user-facing error message should be localized and sent in the
        /// [google.rpc.Status.details][google.rpc.Status.details] field, or
        /// localized by the client.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Status {
        fn default() -> Self {
            Self {
                code: Default::default(),
                details: Default::default(),
                message: Default::default(),
            }
        }
    }

    ///S3 configuration for an S3-compatible object store.
    /// Reference: https://developers.cloudflare.com/r2/examples/aws/aws-sdk-go/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "S3 configuration for an S3-compatible object store.\n Reference: https://developers.cloudflare.com/r2/examples/aws/aws-sdk-go/",
    ///  "type": "object",
    ///  "properties": {
    ///    "accessKeyId": {
    ///      "type": "string"
    ///    },
    ///    "accessKeySecret": {
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "bucket": {
    ///      "type": "string"
    ///    },
    ///    "endpoint": {
    ///      "type": "string"
    ///    },
    ///    "insecureSkipTlsVerify": {
    ///      "description": "insecure_skip_tls_verify disables TLS certificate
    /// verification when connecting\n to the S3 endpoint. Only enable this for
    /// trusted endpoints that use a self-signed\n certificate; it removes
    /// protection against man-in-the-middle attacks.",
    ///      "type": "boolean"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "usePathStyle": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StorageS3Config {
        #[serde(
            rename = "accessKeyId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_key_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "accessKeySecret",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_key_secret: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub endpoint: ::std::option::Option<::std::string::String>,
        ///insecure_skip_tls_verify disables TLS certificate verification when
        /// connecting to the S3 endpoint. Only enable this for trusted
        /// endpoints that use a self-signed certificate; it removes
        /// protection against man-in-the-middle attacks.
        #[serde(
            rename = "insecureSkipTlsVerify",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub insecure_skip_tls_verify: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "usePathStyle",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub use_path_style: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for StorageS3Config {
        fn default() -> Self {
            Self {
                access_key_id: Default::default(),
                access_key_secret: Default::default(),
                bucket: Default::default(),
                endpoint: Default::default(),
                insecure_skip_tls_verify: Default::default(),
                region: Default::default(),
                use_path_style: Default::default(),
            }
        }
    }

    ///Legacy S3 configuration retained for compatibility with existing
    /// clients. Reference: https://developers.cloudflare.com/r2/examples/aws/aws-sdk-go/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Legacy S3 configuration retained for compatibility with existing clients.\n Reference: https://developers.cloudflare.com/r2/examples/aws/aws-sdk-go/",
    ///  "type": "object",
    ///  "properties": {
    ///    "accessKeyId": {
    ///      "type": "string"
    ///    },
    ///    "accessKeySecret": {
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "bucket": {
    ///      "type": "string"
    ///    },
    ///    "endpoint": {
    ///      "type": "string"
    ///    },
    ///    "insecureSkipTlsVerify": {
    ///      "description": "insecure_skip_tls_verify disables TLS certificate
    /// verification when connecting\n to the S3 endpoint. Only enable this for
    /// trusted endpoints that use a self-signed\n certificate; it removes
    /// protection against man-in-the-middle attacks.",
    ///      "type": "boolean"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "usePathStyle": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StorageSettingS3Config {
        #[serde(
            rename = "accessKeyId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_key_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "accessKeySecret",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub access_key_secret: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub endpoint: ::std::option::Option<::std::string::String>,
        ///insecure_skip_tls_verify disables TLS certificate verification when
        /// connecting to the S3 endpoint. Only enable this for trusted
        /// endpoints that use a self-signed certificate; it removes
        /// protection against man-in-the-middle attacks.
        #[serde(
            rename = "insecureSkipTlsVerify",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub insecure_skip_tls_verify: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "usePathStyle",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub use_path_style: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for StorageSettingS3Config {
        fn default() -> Self {
            Self {
                access_key_id: Default::default(),
                access_key_secret: Default::default(),
                bucket: Default::default(),
                endpoint: Default::default(),
                insecure_skip_tls_verify: Default::default(),
                region: Default::default(),
                use_path_style: Default::default(),
            }
        }
    }

    ///Request message for TestInstanceEmailSetting method.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Request message for TestInstanceEmailSetting method.",
    ///  "type": "object",
    ///  "properties": {
    ///    "email": {
    ///      "description": "Optional. SMTP email settings to test. If omitted,
    /// the stored notification email setting is used.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/NotificationSetting_EmailSetting"
    ///        }
    ///      ]
    ///    },
    ///    "recipientEmail": {
    ///      "description": "Optional. Recipient email address. If omitted, the
    /// current user's email address is used.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TestInstanceEmailSettingRequest {
        ///Optional. SMTP email settings to test. If omitted, the stored
        /// notification email setting is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<NotificationSettingEmailSetting>,
        ///Optional. Recipient email address. If omitted, the current user's
        /// email address is used.
        #[serde(
            rename = "recipientEmail",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub recipient_email: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for TestInstanceEmailSettingRequest {
        fn default() -> Self {
            Self {
                email: Default::default(),
                recipient_email: Default::default(),
            }
        }
    }

    ///`TranscribeRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "audio"
    ///  ],
    ///  "properties": {
    ///    "audio": {
    ///      "description": "Required. Audio input.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TranscriptionAudio"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TranscribeRequest {
        ///Required. Audio input.
        pub audio: TranscriptionAudio,
    }

    ///`TranscribeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "text": {
    ///      "description": "The transcribed text.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TranscribeResponse {
        ///The transcribed text.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for TranscribeResponse {
        fn default() -> Self {
            Self {
                text: Default::default(),
            }
        }
    }

    ///`TranscriptionAudio`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "content": {
    ///      "description": "Inline audio bytes.",
    ///      "writeOnly": true,
    ///      "type": "string",
    ///      "format": "bytes"
    ///    },
    ///    "contentType": {
    ///      "description": "Optional. The MIME type of the input audio.",
    ///      "type": "string"
    ///    },
    ///    "filename": {
    ///      "description": "Optional. The uploaded filename.",
    ///      "type": "string"
    ///    },
    ///    "uri": {
    ///      "description": "URI for audio content. Reserved for future use.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TranscriptionAudio {
        ///Inline audio bytes.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        ///Optional. The MIME type of the input audio.
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<::std::string::String>,
        ///Optional. The uploaded filename.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub filename: ::std::option::Option<::std::string::String>,
        ///URI for audio content. Reserved for future use.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub uri: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for TranscriptionAudio {
        fn default() -> Self {
            Self {
                content: Default::default(),
                content_type: Default::default(),
                filename: Default::default(),
                uri: Default::default(),
            }
        }
    }

    ///`UpsertMemoReactionRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "reaction"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "Required. The resource name of the memo.\n Format:
    /// memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "reaction": {
    ///      "description": "Required. The reaction to upsert.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Reaction"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpsertMemoReactionRequest {
        ///Required. The resource name of the memo.
        /// Format: memos/{memo}
        pub name: ::std::string::String,
        ///Required. The reaction to upsert.
        pub reaction: Reaction,
    }

    ///`User`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "role",
    ///    "state",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "avatarUrl": {
    ///      "description": "Optional. The avatar URL of the user.",
    ///      "type": "string"
    ///    },
    ///    "createTime": {
    ///      "description": "Output only. The creation timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "description": {
    ///      "description": "Optional. The description of the user.",
    ///      "type": "string"
    ///    },
    ///    "displayName": {
    ///      "description": "Optional. The display name of the user.",
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "description": "Optional. The email address of the user.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the user.\n Format:
    /// users/{user}",
    ///      "type": "string"
    ///    },
    ///    "password": {
    ///      "description": "Input only. The password for the user.",
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "role": {
    ///      "description": "The role of the user.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "ROLE_UNSPECIFIED",
    ///        "ADMIN",
    ///        "USER"
    ///      ]
    ///    },
    ///    "state": {
    ///      "description": "The state of the user.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "STATE_UNSPECIFIED",
    ///        "NORMAL",
    ///        "ARCHIVED"
    ///      ]
    ///    },
    ///    "updateTime": {
    ///      "description": "Output only. The last update timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "username": {
    ///      "description": "Required. The unique username for login.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct User {
        ///Optional. The avatar URL of the user.
        #[serde(
            rename = "avatarUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        ///Output only. The creation timestamp.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Optional. The description of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Optional. The display name of the user.
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<::std::string::String>,
        ///Optional. The email address of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        ///The resource name of the user.
        /// Format: users/{user}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Input only. The password for the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub password: ::std::option::Option<::std::string::String>,
        ///The role of the user.
        pub role: UserRole,
        ///The state of the user.
        pub state: UserState,
        ///Output only. The last update timestamp.
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Required. The unique username for login.
        pub username: ::std::string::String,
    }

    ///`UserNotification`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "createTime": {
    ///      "description": "The creation timestamp.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "memoComment": {
    ///      "readOnly": true,
    ///      "allOf": [
    ///        {
    ///          "$ref":
    /// "#/components/schemas/UserNotification_MemoCommentPayload"
    ///        }
    ///      ]
    ///    },
    ///    "memoMention": {
    ///      "readOnly": true,
    ///      "allOf": [
    ///        {
    ///          "$ref":
    /// "#/components/schemas/UserNotification_MemoMentionPayload"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the notification.\n Format:
    /// users/{user}/notifications/{notification}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "sender": {
    ///      "description": "The sender of the notification.\n Format:
    /// users/{user}",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "senderUser": {
    ///      "description": "The sender user details.",
    ///      "readOnly": true,
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/User"
    ///        }
    ///      ]
    ///    },
    ///    "status": {
    ///      "description": "The status of the notification.",
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "STATUS_UNSPECIFIED",
    ///        "UNREAD",
    ///        "ARCHIVED"
    ///      ]
    ///    },
    ///    "type": {
    ///      "description": "The type of the notification.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "enum",
    ///      "enum": [
    ///        "TYPE_UNSPECIFIED",
    ///        "MEMO_COMMENT",
    ///        "MEMO_MENTION"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserNotification {
        ///The creation timestamp.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "memoComment",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_comment: ::std::option::Option<UserNotificationMemoCommentPayload>,
        #[serde(
            rename = "memoMention",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_mention: ::std::option::Option<UserNotificationMemoMentionPayload>,
        ///The resource name of the notification.
        /// Format: users/{user}/notifications/{notification}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The sender of the notification.
        /// Format: users/{user}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sender: ::std::option::Option<::std::string::String>,
        ///The sender user details.
        #[serde(
            rename = "senderUser",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sender_user: ::std::option::Option<User>,
        ///The status of the notification.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<UserNotificationStatus>,
        ///The type of the notification.
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<UserNotificationType>,
    }

    impl ::std::default::Default for UserNotification {
        fn default() -> Self {
            Self {
                create_time: Default::default(),
                memo_comment: Default::default(),
                memo_mention: Default::default(),
                name: Default::default(),
                sender: Default::default(),
                sender_user: Default::default(),
                status: Default::default(),
                type_: Default::default(),
            }
        }
    }

    ///`UserNotificationMemoCommentPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memo": {
    ///      "description": "The memo name of comment.\n Format: memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "memoSnippet": {
    ///      "description": "Preview text of the comment memo.",
    ///      "type": "string"
    ///    },
    ///    "relatedMemo": {
    ///      "description": "The name of related memo.\n Format: memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "relatedMemoSnippet": {
    ///      "description": "Preview text of the related memo.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserNotificationMemoCommentPayload {
        ///The memo name of comment.
        /// Format: memos/{memo}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub memo: ::std::option::Option<::std::string::String>,
        ///Preview text of the comment memo.
        #[serde(
            rename = "memoSnippet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_snippet: ::std::option::Option<::std::string::String>,
        ///The name of related memo.
        /// Format: memos/{memo}
        #[serde(
            rename = "relatedMemo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub related_memo: ::std::option::Option<::std::string::String>,
        ///Preview text of the related memo.
        #[serde(
            rename = "relatedMemoSnippet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub related_memo_snippet: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UserNotificationMemoCommentPayload {
        fn default() -> Self {
            Self {
                memo: Default::default(),
                memo_snippet: Default::default(),
                related_memo: Default::default(),
                related_memo_snippet: Default::default(),
            }
        }
    }

    ///`UserNotificationMemoMentionPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memo": {
    ///      "description": "The memo that contains the mention.\n Format:
    /// memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "memoSnippet": {
    ///      "description": "Preview text of the memo that contains the
    /// mention.",
    ///      "type": "string"
    ///    },
    ///    "relatedMemo": {
    ///      "description": "The related parent memo when the mention was
    /// created in a comment.\n Format: memos/{memo}",
    ///      "type": "string"
    ///    },
    ///    "relatedMemoSnippet": {
    ///      "description": "Preview text of the related parent memo.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserNotificationMemoMentionPayload {
        ///The memo that contains the mention.
        /// Format: memos/{memo}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub memo: ::std::option::Option<::std::string::String>,
        ///Preview text of the memo that contains the mention.
        #[serde(
            rename = "memoSnippet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_snippet: ::std::option::Option<::std::string::String>,
        ///The related parent memo when the mention was created in a comment.
        /// Format: memos/{memo}
        #[serde(
            rename = "relatedMemo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub related_memo: ::std::option::Option<::std::string::String>,
        ///Preview text of the related parent memo.
        #[serde(
            rename = "relatedMemoSnippet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub related_memo_snippet: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UserNotificationMemoMentionPayload {
        fn default() -> Self {
            Self {
                memo: Default::default(),
                memo_snippet: Default::default(),
                related_memo: Default::default(),
                related_memo_snippet: Default::default(),
            }
        }
    }

    ///The status of the notification.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The status of the notification.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STATUS_UNSPECIFIED",
    ///    "UNREAD",
    ///    "ARCHIVED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserNotificationStatus {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        StatusUnspecified,
        #[serde(rename = "UNREAD")]
        Unread,
        #[serde(rename = "ARCHIVED")]
        Archived,
    }

    impl ::std::fmt::Display for UserNotificationStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StatusUnspecified => f.write_str("STATUS_UNSPECIFIED"),
                Self::Unread => f.write_str("UNREAD"),
                Self::Archived => f.write_str("ARCHIVED"),
            }
        }
    }

    impl ::std::str::FromStr for UserNotificationStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STATUS_UNSPECIFIED" => Ok(Self::StatusUnspecified),
                "UNREAD" => Ok(Self::Unread),
                "ARCHIVED" => Ok(Self::Archived),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UserNotificationStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserNotificationStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserNotificationStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The type of the notification.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The type of the notification.",
    ///  "readOnly": true,
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "TYPE_UNSPECIFIED",
    ///    "MEMO_COMMENT",
    ///    "MEMO_MENTION"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserNotificationType {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        TypeUnspecified,
        #[serde(rename = "MEMO_COMMENT")]
        MemoComment,
        #[serde(rename = "MEMO_MENTION")]
        MemoMention,
    }

    impl ::std::fmt::Display for UserNotificationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::TypeUnspecified => f.write_str("TYPE_UNSPECIFIED"),
                Self::MemoComment => f.write_str("MEMO_COMMENT"),
                Self::MemoMention => f.write_str("MEMO_MENTION"),
            }
        }
    }

    impl ::std::str::FromStr for UserNotificationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TYPE_UNSPECIFIED" => Ok(Self::TypeUnspecified),
                "MEMO_COMMENT" => Ok(Self::MemoComment),
                "MEMO_MENTION" => Ok(Self::MemoMention),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UserNotificationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserNotificationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserNotificationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The role of the user.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The role of the user.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "ROLE_UNSPECIFIED",
    ///    "ADMIN",
    ///    "USER"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserRole {
        #[serde(rename = "ROLE_UNSPECIFIED")]
        RoleUnspecified,
        #[serde(rename = "ADMIN")]
        Admin,
        #[serde(rename = "USER")]
        User,
    }

    impl ::std::fmt::Display for UserRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::RoleUnspecified => f.write_str("ROLE_UNSPECIFIED"),
                Self::Admin => f.write_str("ADMIN"),
                Self::User => f.write_str("USER"),
            }
        }
    }

    impl ::std::str::FromStr for UserRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ROLE_UNSPECIFIED" => Ok(Self::RoleUnspecified),
                "ADMIN" => Ok(Self::Admin),
                "USER" => Ok(Self::User),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UserRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`UserServiceListAllUserStatsState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STATE_UNSPECIFIED",
    ///    "NORMAL",
    ///    "ARCHIVED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserServiceListAllUserStatsState {
        #[serde(rename = "STATE_UNSPECIFIED")]
        StateUnspecified,
        #[serde(rename = "NORMAL")]
        Normal,
        #[serde(rename = "ARCHIVED")]
        Archived,
    }

    impl ::std::fmt::Display for UserServiceListAllUserStatsState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StateUnspecified => f.write_str("STATE_UNSPECIFIED"),
                Self::Normal => f.write_str("NORMAL"),
                Self::Archived => f.write_str("ARCHIVED"),
            }
        }
    }

    impl ::std::str::FromStr for UserServiceListAllUserStatsState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STATE_UNSPECIFIED" => Ok(Self::StateUnspecified),
                "NORMAL" => Ok(Self::Normal),
                "ARCHIVED" => Ok(Self::Archived),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UserServiceListAllUserStatsState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserServiceListAllUserStatsState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserServiceListAllUserStatsState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///User settings message
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "User settings message",
    ///  "type": "object",
    ///  "properties": {
    ///    "generalSetting": {
    ///      "$ref": "#/components/schemas/UserSetting_GeneralSetting"
    ///    },
    ///    "name": {
    ///      "description": "The name of the user setting.\n Format: users/{user}/settings/{setting}, {setting} is the key for the setting.\n For example, \"users/steven/settings/GENERAL\" for general settings.",
    ///      "type": "string"
    ///    },
    ///    "tagsSetting": {
    ///      "$ref": "#/components/schemas/UserSetting_TagsSetting"
    ///    },
    ///    "webhooksSetting": {
    ///      "$ref": "#/components/schemas/UserSetting_WebhooksSetting"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserSetting {
        #[serde(
            rename = "generalSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub general_setting: ::std::option::Option<UserSettingGeneralSetting>,
        ///The name of the user setting.
        /// Format: users/{user}/settings/{setting}, {setting} is the key for
        /// the setting. For example, "users/steven/settings/GENERAL"
        /// for general settings.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "tagsSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tags_setting: ::std::option::Option<UserSettingTagsSetting>,
        #[serde(
            rename = "webhooksSetting",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub webhooks_setting: ::std::option::Option<UserSettingWebhooksSetting>,
    }

    impl ::std::default::Default for UserSetting {
        fn default() -> Self {
            Self {
                general_setting: Default::default(),
                name: Default::default(),
                tags_setting: Default::default(),
                webhooks_setting: Default::default(),
            }
        }
    }

    ///General user settings configuration.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "General user settings configuration.",
    ///  "type": "object",
    ///  "properties": {
    ///    "locale": {
    ///      "description": "The preferred locale of the user.",
    ///      "type": "string"
    ///    },
    ///    "memoVisibility": {
    ///      "description": "The default visibility of the memo.",
    ///      "type": "string"
    ///    },
    ///    "saveMediaMetadata": {
    ///      "description": "Whether the official client should save metadata
    /// from future media uploads.",
    ///      "type": "boolean"
    ///    },
    ///    "theme": {
    ///      "description": "The preferred theme of the user.\n This references
    /// a CSS file in the web/public/themes/ directory.\n If not set, the
    /// default theme will be used.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserSettingGeneralSetting {
        ///The preferred locale of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locale: ::std::option::Option<::std::string::String>,
        ///The default visibility of the memo.
        #[serde(
            rename = "memoVisibility",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_visibility: ::std::option::Option<::std::string::String>,
        ///Whether the official client should save metadata from future media
        /// uploads.
        #[serde(
            rename = "saveMediaMetadata",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub save_media_metadata: ::std::option::Option<bool>,
        ///The preferred theme of the user.
        /// This references a CSS file in the web/public/themes/ directory.
        /// If not set, the default theme will be used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub theme: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UserSettingGeneralSetting {
        fn default() -> Self {
            Self {
                locale: Default::default(),
                memo_visibility: Default::default(),
                save_media_metadata: Default::default(),
                theme: Default::default(),
            }
        }
    }

    ///Tag metadata for user-specific display rules.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Tag metadata for user-specific display rules.",
    ///  "type": "object",
    ///  "properties": {
    ///    "backgroundColor": {
    ///      "description": "Optional background color for the tag label.\n When
    /// unset, the default tag color is used.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Color"
    ///        }
    ///      ]
    ///    },
    ///    "blurContent": {
    ///      "description": "Whether memos with this tag should have their
    /// content blurred.",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserSettingTagMetadata {
        ///Optional background color for the tag label.
        /// When unset, the default tag color is used.
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<Color>,
        ///Whether memos with this tag should have their content blurred.
        #[serde(
            rename = "blurContent",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub blur_content: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for UserSettingTagMetadata {
        fn default() -> Self {
            Self {
                background_color: Default::default(),
                blur_content: Default::default(),
            }
        }
    }

    ///User-specific tag metadata.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "User-specific tag metadata.",
    ///  "type": "object",
    ///  "properties": {
    ///    "tags": {
    ///      "description": "Map of tag name pattern to tag metadata.\n Each key
    /// is treated as an anchored regular expression (^pattern$).",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/UserSetting_TagMetadata"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserSettingTagsSetting {
        ///Map of tag name pattern to tag metadata.
        /// Each key is treated as an anchored regular expression (^pattern$).
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub tags: ::std::collections::HashMap<::std::string::String, UserSettingTagMetadata>,
    }

    impl ::std::default::Default for UserSettingTagsSetting {
        fn default() -> Self {
            Self {
                tags: Default::default(),
            }
        }
    }

    ///User webhooks configuration.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "User webhooks configuration.",
    ///  "type": "object",
    ///  "properties": {
    ///    "webhooks": {
    ///      "description": "List of user webhooks.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserWebhook"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserSettingWebhooksSetting {
        ///List of user webhooks.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub webhooks: ::std::vec::Vec<UserWebhook>,
    }

    impl ::std::default::Default for UserSettingWebhooksSetting {
        fn default() -> Self {
            Self {
                webhooks: Default::default(),
            }
        }
    }

    ///The state of the user.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The state of the user.",
    ///  "type": "string",
    ///  "format": "enum",
    ///  "enum": [
    ///    "STATE_UNSPECIFIED",
    ///    "NORMAL",
    ///    "ARCHIVED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UserState {
        #[serde(rename = "STATE_UNSPECIFIED")]
        StateUnspecified,
        #[serde(rename = "NORMAL")]
        Normal,
        #[serde(rename = "ARCHIVED")]
        Archived,
    }

    impl ::std::fmt::Display for UserState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::StateUnspecified => f.write_str("STATE_UNSPECIFIED"),
                Self::Normal => f.write_str("NORMAL"),
                Self::Archived => f.write_str("ARCHIVED"),
            }
        }
    }

    impl ::std::str::FromStr for UserState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STATE_UNSPECIFIED" => Ok(Self::StateUnspecified),
                "NORMAL" => Ok(Self::Normal),
                "ARCHIVED" => Ok(Self::Archived),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UserState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///User statistics messages
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "User statistics messages",
    ///  "type": "object",
    ///  "properties": {
    ///    "memoCreatedTimestamps": {
    ///      "description": "The creation timestamps of the user's memos.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "format": "date-time"
    ///      }
    ///    },
    ///    "memoTypeStats": {
    ///      "description": "The stats of memo types.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/UserStats_MemoTypeStats"
    ///        }
    ///      ]
    ///    },
    ///    "memoUpdatedTimestamps": {
    ///      "description": "The latest update timestamps of the user's memos
    /// (one per memo,\n mirrors memo_created_timestamps). Used by the activity
    /// heatmap when\n the client's view is set to update_time basis.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "format": "date-time"
    ///      }
    ///    },
    ///    "name": {
    ///      "description": "The resource name of the user statistics
    /// singleton.\n Format: users/{user}/stats",
    ///      "type": "string"
    ///    },
    ///    "pinnedMemos": {
    ///      "description": "The pinned memos of the user.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "tagCount": {
    ///      "description": "The count of tags.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "integer",
    ///        "format": "int32"
    ///      }
    ///    },
    ///    "totalMemoCount": {
    ///      "description": "Total memo count.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserStats {
        ///The creation timestamps of the user's memos.
        #[serde(
            rename = "memoCreatedTimestamps",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub memo_created_timestamps: ::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The stats of memo types.
        #[serde(
            rename = "memoTypeStats",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub memo_type_stats: ::std::option::Option<UserStatsMemoTypeStats>,
        ///The latest update timestamps of the user's memos (one per memo,
        /// mirrors memo_created_timestamps). Used by the activity heatmap when
        /// the client's view is set to update_time basis.
        #[serde(
            rename = "memoUpdatedTimestamps",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub memo_updated_timestamps: ::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The resource name of the user statistics singleton.
        /// Format: users/{user}/stats
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The pinned memos of the user.
        #[serde(
            rename = "pinnedMemos",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub pinned_memos: ::std::vec::Vec<::std::string::String>,
        ///The count of tags.
        #[serde(
            rename = "tagCount",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub tag_count: ::std::collections::HashMap<::std::string::String, i32>,
        ///Total memo count.
        #[serde(
            rename = "totalMemoCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_memo_count: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for UserStats {
        fn default() -> Self {
            Self {
                memo_created_timestamps: Default::default(),
                memo_type_stats: Default::default(),
                memo_updated_timestamps: Default::default(),
                name: Default::default(),
                pinned_memos: Default::default(),
                tag_count: Default::default(),
                total_memo_count: Default::default(),
            }
        }
    }

    ///Memo type statistics.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Memo type statistics.",
    ///  "type": "object",
    ///  "properties": {
    ///    "codeCount": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "linkCount": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "todoCount": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "undoCount": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserStatsMemoTypeStats {
        #[serde(
            rename = "codeCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub code_count: ::std::option::Option<i32>,
        #[serde(
            rename = "linkCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub link_count: ::std::option::Option<i32>,
        #[serde(
            rename = "todoCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub todo_count: ::std::option::Option<i32>,
        #[serde(
            rename = "undoCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub undo_count: ::std::option::Option<i32>,
    }

    impl ::std::default::Default for UserStatsMemoTypeStats {
        fn default() -> Self {
            Self {
                code_count: Default::default(),
                link_count: Default::default(),
                todo_count: Default::default(),
                undo_count: Default::default(),
            }
        }
    }

    ///UserWebhook represents a webhook owned by a user.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "UserWebhook represents a webhook owned by a user.",
    ///  "type": "object",
    ///  "properties": {
    ///    "createTime": {
    ///      "description": "The creation time of the webhook.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "displayName": {
    ///      "description": "Optional. Human-readable name for the webhook.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the webhook.\n Format:
    /// users/{user}/webhooks/{webhook}",
    ///      "type": "string"
    ///    },
    ///    "signingSecret": {
    ///      "description": "Optional. Signing secret used to HMAC-SHA256 sign
    /// the webhook request body.\n This field is input-only; it is never
    /// returned in responses.",
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "signingSecretSet": {
    ///      "description": "Whether a signing secret is configured for this
    /// webhook.",
    ///      "readOnly": true,
    ///      "type": "boolean"
    ///    },
    ///    "updateTime": {
    ///      "description": "The last update time of the webhook.",
    ///      "readOnly": true,
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "url": {
    ///      "description": "The URL to send the webhook to.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserWebhook {
        ///The creation time of the webhook.
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Optional. Human-readable name for the webhook.
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<::std::string::String>,
        ///The name of the webhook.
        /// Format: users/{user}/webhooks/{webhook}
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Optional. Signing secret used to HMAC-SHA256 sign the webhook
        /// request body. This field is input-only; it is never returned
        /// in responses.
        #[serde(
            rename = "signingSecret",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub signing_secret: ::std::option::Option<::std::string::String>,
        ///Whether a signing secret is configured for this webhook.
        #[serde(
            rename = "signingSecretSet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub signing_secret_set: ::std::option::Option<bool>,
        ///The last update time of the webhook.
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The URL to send the webhook to.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UserWebhook {
        fn default() -> Self {
            Self {
                create_time: Default::default(),
                display_name: Default::default(),
                name: Default::default(),
                signing_secret: Default::default(),
                signing_secret_set: Default::default(),
                update_time: Default::default(),
                url: Default::default(),
            }
        }
    }

    ///`VideoMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "durationSeconds": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VideoMetadata {
        #[serde(
            rename = "durationSeconds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub duration_seconds: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for VideoMetadata {
        fn default() -> Self {
            Self {
                duration_seconds: Default::default(),
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for
///
///Version: 0.0.1
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.0.1"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Transcribe transcribes an audio file using an instance AI provider.
    ///
    ///Sends a `POST` request to `/api/v1/ai:transcribe`
    pub async fn ai_service_transcribe<'a>(
        &'a self,
        body: &'a types::TranscribeRequest,
    ) -> Result<ResponseValue<types::TranscribeResponse>, Error<()>> {
        let url = format!("{}/api/v1/ai:transcribe", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ai_service_transcribe",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListAttachments lists all attachments.
    ///
    ///Sends a `GET` request to `/api/v1/attachments`
    ///
    ///Arguments:
    /// - `filter`: Optional. Filter to apply to the list results.
    /// Example: "mime_type==\"image/png\"" or "filename.contains(\"test\")"
    /// Supported operators: =, !=, <, <=, >, >=, : (contains), in
    /// Supported fields: filename, mime_type, create_time, memo
    /// - `order_by`: Optional. The order to sort results by.
    /// Example: "create_time desc" or "filename asc"
    /// - `page_size`: Optional. The maximum number of attachments to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 attachments will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    /// - `page_token`: Optional. A page token, received from a previous
    ///   `ListAttachments` call.
    /// Provide this to retrieve the subsequent page.
    pub async fn attachment_service_list_attachments<'a>(
        &'a self,
        filter: Option<&'a str>,
        order_by: Option<&'a str>,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListAttachmentsResponse>, Error<()>> {
        let url = format!("{}/api/v1/attachments", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("orderBy", &order_by))
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attachment_service_list_attachments",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateAttachment creates a new attachment.
    ///
    ///Sends a `POST` request to `/api/v1/attachments`
    ///
    ///Arguments:
    /// - `attachment_id`: Optional. The attachment ID to use for this
    ///   attachment.
    /// If empty, a unique ID will be generated.
    /// Format: ^[a-zA-Z0-9]([a-zA-Z0-9-]{0,34}[a-zA-Z0-9])?$
    /// - `body`
    pub async fn attachment_service_create_attachment<'a>(
        &'a self,
        attachment_id: Option<&'a str>,
        body: &'a types::Attachment,
    ) -> Result<ResponseValue<types::Attachment>, Error<()>> {
        let url = format!("{}/api/v1/attachments", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "attachmentId",
                &attachment_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attachment_service_create_attachment",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetAttachment returns an attachment by name.
    ///
    ///Sends a `GET` request to `/api/v1/attachments/{attachment}`
    ///
    ///Arguments:
    /// - `attachment`: The attachment id.
    pub async fn attachment_service_get_attachment<'a>(
        &'a self,
        attachment: &'a str,
    ) -> Result<ResponseValue<types::Attachment>, Error<()>> {
        let url = format!(
            "{}/api/v1/attachments/{}",
            self.baseurl,
            encode_path(&attachment.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attachment_service_get_attachment",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteAttachment deletes an attachment by name.
    ///
    ///Sends a `DELETE` request to `/api/v1/attachments/{attachment}`
    ///
    ///Arguments:
    /// - `attachment`: The attachment id.
    pub async fn attachment_service_delete_attachment<'a>(
        &'a self,
        attachment: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/attachments/{}",
            self.baseurl,
            encode_path(&attachment.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "attachment_service_delete_attachment",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateAttachment updates an attachment.
    ///
    ///Sends a `PATCH` request to `/api/v1/attachments/{attachment}`
    ///
    ///Arguments:
    /// - `attachment`: The attachment id.
    /// - `update_mask`: Required. The list of fields to update.
    /// - `body`
    pub async fn attachment_service_update_attachment<'a>(
        &'a self,
        attachment: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::Attachment,
    ) -> Result<ResponseValue<types::Attachment>, Error<()>> {
        let url = format!(
            "{}/api/v1/attachments/{}",
            self.baseurl,
            encode_path(&attachment.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attachment_service_update_attachment",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///BatchDeleteAttachments deletes multiple attachments in one request.
    ///
    ///Sends a `POST` request to `/api/v1/attachments:batchDelete`
    pub async fn attachment_service_batch_delete_attachments<'a>(
        &'a self,
        body: &'a types::BatchDeleteAttachmentsRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/api/v1/attachments:batchDelete", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attachment_service_batch_delete_attachments",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetCurrentUser returns the authenticated user's information.
    /// Validates the access token and returns user details.
    /// Similar to OIDC's /userinfo endpoint.
    ///
    ///Sends a `GET` request to `/api/v1/auth/me`
    pub async fn auth_service_get_current_user<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetCurrentUserResponse>, Error<()>> {
        let url = format!("{}/api/v1/auth/me", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "auth_service_get_current_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///RefreshToken exchanges a valid refresh token for a new access token.
    /// The refresh token is read from the HttpOnly cookie.
    /// Returns a new short-lived access token.
    ///
    ///Sends a `POST` request to `/api/v1/auth/refresh`
    pub async fn auth_service_refresh_token<'a>(
        &'a self,
        body: &'a types::RefreshTokenRequest,
    ) -> Result<ResponseValue<types::RefreshTokenResponse>, Error<()>> {
        let url = format!("{}/api/v1/auth/refresh", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "auth_service_refresh_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///SignIn authenticates a user with credentials and returns tokens.
    /// On success, returns an access token and sets a refresh token cookie.
    /// Supports password-based and SSO authentication methods.
    ///
    ///Sends a `POST` request to `/api/v1/auth/signin`
    pub async fn auth_service_sign_in<'a>(
        &'a self,
        body: &'a types::SignInRequest,
    ) -> Result<ResponseValue<types::SignInResponse>, Error<()>> {
        let url = format!("{}/api/v1/auth/signin", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "auth_service_sign_in",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///SignOut terminates the user's authentication.
    /// Revokes the refresh token and clears the authentication cookie.
    ///
    ///Sends a `POST` request to `/api/v1/auth/signout`
    pub async fn auth_service_sign_out<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/api/v1/auth/signout", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "auth_service_sign_out",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListIdentityProviders lists identity providers.
    ///
    ///Sends a `GET` request to `/api/v1/identity-providers`
    pub async fn identity_provider_service_list_identity_providers<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ListIdentityProvidersResponse>, Error<()>> {
        let url = format!("{}/api/v1/identity-providers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "identity_provider_service_list_identity_providers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateIdentityProvider creates an identity provider.
    ///
    ///Sends a `POST` request to `/api/v1/identity-providers`
    ///
    ///Arguments:
    /// - `identity_provider_id`: Optional. The ID to use for the identity
    ///   provider, which will become the final component of the resource name.
    /// If not provided, the system will generate one.
    /// Format: ^[a-zA-Z0-9]([a-zA-Z0-9-]{0,34}[a-zA-Z0-9])?$
    /// - `body`
    pub async fn identity_provider_service_create_identity_provider<'a>(
        &'a self,
        identity_provider_id: Option<&'a str>,
        body: &'a types::IdentityProvider,
    ) -> Result<ResponseValue<types::IdentityProvider>, Error<()>> {
        let url = format!("{}/api/v1/identity-providers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "identityProviderId",
                &identity_provider_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "identity_provider_service_create_identity_provider",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetIdentityProvider gets an identity provider.
    ///
    ///Sends a `GET` request to
    /// `/api/v1/identity-providers/{identity-provider}`
    ///
    ///Arguments:
    /// - `identity_provider`: The identity-provider id.
    pub async fn identity_provider_service_get_identity_provider<'a>(
        &'a self,
        identity_provider: &'a str,
    ) -> Result<ResponseValue<types::IdentityProvider>, Error<()>> {
        let url = format!(
            "{}/api/v1/identity-providers/{}",
            self.baseurl,
            encode_path(&identity_provider.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "identity_provider_service_get_identity_provider",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteIdentityProvider deletes an identity provider.
    ///
    ///Sends a `DELETE` request to
    /// `/api/v1/identity-providers/{identity-provider}`
    ///
    ///Arguments:
    /// - `identity_provider`: The identity-provider id.
    pub async fn identity_provider_service_delete_identity_provider<'a>(
        &'a self,
        identity_provider: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/identity-providers/{}",
            self.baseurl,
            encode_path(&identity_provider.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "identity_provider_service_delete_identity_provider",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateIdentityProvider updates an identity provider.
    ///
    ///Sends a `PATCH` request to
    /// `/api/v1/identity-providers/{identity-provider}`
    ///
    ///Arguments:
    /// - `identity_provider`: The identity-provider id.
    /// - `update_mask`: Required. The update mask applies to the resource. Only
    ///   the top level fields of
    /// IdentityProvider are supported.
    /// - `body`
    pub async fn identity_provider_service_update_identity_provider<'a>(
        &'a self,
        identity_provider: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::IdentityProvider,
    ) -> Result<ResponseValue<types::IdentityProvider>, Error<()>> {
        let url = format!(
            "{}/api/v1/identity-providers/{}",
            self.baseurl,
            encode_path(&identity_provider.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "identity_provider_service_update_identity_provider",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Gets the instance profile.
    ///
    ///Sends a `GET` request to `/api/v1/instance/profile`
    pub async fn instance_service_get_instance_profile<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::InstanceProfile>, Error<()>> {
        let url = format!("{}/api/v1/instance/profile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_service_get_instance_profile",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Tests notification email delivery with the provided or stored SMTP
    /// settings.
    ///
    ///Sends a `POST` request to
    /// `/api/v1/instance/settings/notification:testEmail`
    pub async fn instance_service_test_instance_email_setting<'a>(
        &'a self,
        body: &'a types::TestInstanceEmailSettingRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/instance/settings/notification:testEmail",
            self.baseurl,
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_service_test_instance_email_setting",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Batch gets instance settings.
    ///
    ///Sends a `POST` request to `/api/v1/instance/settings:batchGet`
    pub async fn instance_service_batch_get_instance_settings<'a>(
        &'a self,
        body: &'a types::BatchGetInstanceSettingsRequest,
    ) -> Result<ResponseValue<types::BatchGetInstanceSettingsResponse>, Error<()>> {
        let url = format!("{}/api/v1/instance/settings:batchGet", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_service_batch_get_instance_settings",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetInstanceStats returns resource usage statistics for the instance.
    /// Admin only.
    ///
    ///Sends a `GET` request to `/api/v1/instance/stats`
    pub async fn instance_service_get_instance_stats<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::InstanceStats>, Error<()>> {
        let url = format!("{}/api/v1/instance/stats", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_service_get_instance_stats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Gets an instance setting.
    ///
    ///Sends a `GET` request to `/api/v1/instance/{instance}/*`
    ///
    ///Arguments:
    /// - `instance`: The instance id.
    pub async fn instance_service_get_instance_setting<'a>(
        &'a self,
        instance: &'a str,
    ) -> Result<ResponseValue<types::InstanceSetting>, Error<()>> {
        let url = format!(
            "{}/api/v1/instance/{}/*",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_service_get_instance_setting",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Updates an instance setting.
    ///
    ///Sends a `PATCH` request to `/api/v1/instance/{instance}/*`
    ///
    ///Arguments:
    /// - `instance`: The instance id.
    /// - `update_mask`: The list of fields to update.
    /// - `body`
    pub async fn instance_service_update_instance_setting<'a>(
        &'a self,
        instance: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::InstanceSetting,
    ) -> Result<ResponseValue<types::InstanceSetting>, Error<()>> {
        let url = format!(
            "{}/api/v1/instance/{}/*",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_service_update_instance_setting",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemos lists memos with pagination and filter.
    ///
    ///Sends a `GET` request to `/api/v1/memos`
    ///
    ///Arguments:
    /// - `filter`: Optional. A CEL expression to filter memos. Combine terms
    ///   with && and ||.
    /// Available fields:
    ///   content (string), creator (string, e.g. "users/1"),
    ///   created_ts / updated_ts (timestamp), pinned (bool),
    ///   visibility (string: PRIVATE | PROTECTED | PUBLIC),
    ///   tags (list<string>; match with `"work" in tags`, not `tag == "work"`),
    ///   has_task_list / has_link / has_code / has_incomplete_tasks (bool),
    ///   has_location (bool; true when the memo has a location attached).
    /// Note: the time fields here are created_ts / updated_ts, which differ
    /// from the create_time / update_time names used by order_by.
    /// Examples:
    ///   pinned == true && visibility == "PUBLIC"
    ///   tags.exists(t, t == "urgent")
    ///   content.contains("roadmap") && created_ts > now - duration("168h")
    /// - `order_by`: Optional. The order to sort results by.
    /// Default to "create_time desc".
    /// Supports comma-separated list of fields following AIP-132.
    /// Example: "pinned desc, create_time desc" or "update_time asc"
    /// Supported fields: pinned, create_time, update_time, name.
    /// Note: order_by uses create_time / update_time, while the filter
    /// expression uses created_ts / updated_ts for the same timestamps.
    /// - `page_size`: Optional. The maximum number of memos to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 memos will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    /// - `page_token`: Optional. A page token, received from a previous
    ///   `ListMemos` call.
    /// Provide this to retrieve the subsequent page.
    /// - `show_deleted`: Optional. If true, show deleted memos in the response.
    /// - `state`: Optional. The state of the memos to list.
    /// Default to `NORMAL`. Set to `ARCHIVED` to list archived memos.
    pub async fn memo_service_list_memos<'a>(
        &'a self,
        filter: Option<&'a str>,
        order_by: Option<&'a str>,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
        show_deleted: Option<bool>,
        state: Option<types::MemoServiceListMemosState>,
    ) -> Result<ResponseValue<types::ListMemosResponse>, Error<()>> {
        let url = format!("{}/api/v1/memos", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("orderBy", &order_by))
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "showDeleted",
                &show_deleted,
            ))
            .query(&progenitor_client::QueryParam::new("state", &state))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_list_memos",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateMemo creates a memo. The request body is a Memo; set its content
    /// (Markdown) and visibility (PRIVATE | PROTECTED | PUBLIC, default
    /// PRIVATE). The memo is owned by the authenticated user; requires
    /// authentication.
    ///
    ///Sends a `POST` request to `/api/v1/memos`
    ///
    ///Arguments:
    /// - `memo_id`: Optional. The memo ID to use for this memo.
    /// If empty, a unique ID will be generated.
    /// Format: ^[a-zA-Z0-9]([a-zA-Z0-9-]{0,34}[a-zA-Z0-9])?$
    /// - `body`
    pub async fn memo_service_create_memo<'a>(
        &'a self,
        memo_id: Option<&'a str>,
        body: &'a types::Memo,
    ) -> Result<ResponseValue<types::Memo>, Error<()>> {
        let url = format!("{}/api/v1/memos", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("memoId", &memo_id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_create_memo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetLinkMetadata gets metadata for a link.
    ///
    ///Sends a `GET` request to `/api/v1/memos/-/linkMetadata`
    ///
    ///Arguments:
    /// - `url`: Required. The link URL.
    pub async fn memo_service_get_link_metadata<'a>(
        &'a self,
        url: Option<&'a str>,
    ) -> Result<ResponseValue<types::LinkMetadata>, Error<()>> {
        let _url = format!("{}/api/v1/memos/-/linkMetadata", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(_url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("url", &url))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_get_link_metadata",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///BatchGetLinkMetadata gets metadata for links.
    ///
    ///Sends a `POST` request to `/api/v1/memos/-/linkMetadata:batchGet`
    pub async fn memo_service_batch_get_link_metadata<'a>(
        &'a self,
        body: &'a types::BatchGetLinkMetadataRequest,
    ) -> Result<ResponseValue<types::BatchGetLinkMetadataResponse>, Error<()>> {
        let url = format!("{}/api/v1/memos/-/linkMetadata:batchGet", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_batch_get_link_metadata",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetMemo gets a memo.
    ///
    ///Sends a `GET` request to `/api/v1/memos/{memo}`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    pub async fn memo_service_get_memo<'a>(
        &'a self,
        memo: &'a str,
    ) -> Result<ResponseValue<types::Memo>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_get_memo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteMemo deletes a memo.
    ///
    ///Sends a `DELETE` request to `/api/v1/memos/{memo}`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `force`: Optional. If set to true, the memo will be deleted even if it
    ///   has associated data.
    pub async fn memo_service_delete_memo<'a>(
        &'a self,
        memo: &'a str,
        force: Option<bool>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .query(&progenitor_client::QueryParam::new("force", &force))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_delete_memo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateMemo updates a memo.
    ///
    ///Sends a `PATCH` request to `/api/v1/memos/{memo}`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `update_mask`: Required. The list of fields to update.
    /// - `body`
    pub async fn memo_service_update_memo<'a>(
        &'a self,
        memo: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::Memo,
    ) -> Result<ResponseValue<types::Memo>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_update_memo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemoAttachments lists attachments for a memo.
    ///
    ///Sends a `GET` request to `/api/v1/memos/{memo}/attachments`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `page_size`: Optional. The maximum number of attachments to return.
    /// - `page_token`: Optional. A page token for pagination.
    pub async fn memo_service_list_memo_attachments<'a>(
        &'a self,
        memo: &'a str,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListMemoAttachmentsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/attachments",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_list_memo_attachments",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///SetMemoAttachments replaces the full set of attachments on a memo with
    /// the provided list (not an append). Pass the complete desired set; an
    /// empty list clears all attachments. Idempotent.
    ///
    ///Sends a `PATCH` request to `/api/v1/memos/{memo}/attachments`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `body`
    pub async fn memo_service_set_memo_attachments<'a>(
        &'a self,
        memo: &'a str,
        body: &'a types::SetMemoAttachmentsRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/attachments",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_set_memo_attachments",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemoComments lists comments for a memo.
    ///
    ///Sends a `GET` request to `/api/v1/memos/{memo}/comments`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `order_by`: Optional. The order to sort results by.
    /// - `page_size`: Optional. The maximum number of comments to return.
    /// - `page_token`: Optional. A page token for pagination.
    pub async fn memo_service_list_memo_comments<'a>(
        &'a self,
        memo: &'a str,
        order_by: Option<&'a str>,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListMemoCommentsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/comments",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("orderBy", &order_by))
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_list_memo_comments",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateMemoComment creates a comment for a memo.
    ///
    ///Sends a `POST` request to `/api/v1/memos/{memo}/comments`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `comment_id`: Optional. The comment ID to use.
    /// - `body`
    pub async fn memo_service_create_memo_comment<'a>(
        &'a self,
        memo: &'a str,
        comment_id: Option<&'a str>,
        body: &'a types::Memo,
    ) -> Result<ResponseValue<types::Memo>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/comments",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "commentId",
                &comment_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_create_memo_comment",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemoReactions lists reactions for a memo.
    ///
    ///Sends a `GET` request to `/api/v1/memos/{memo}/reactions`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `page_size`: Optional. The maximum number of reactions to return.
    /// - `page_token`: Optional. A page token for pagination.
    pub async fn memo_service_list_memo_reactions<'a>(
        &'a self,
        memo: &'a str,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListMemoReactionsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/reactions",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_list_memo_reactions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpsertMemoReaction adds or updates the authenticated user's reaction on
    /// a memo. The reaction's content_id is the memo's resource name
    /// (memos/{memo}).
    ///
    ///Sends a `POST` request to `/api/v1/memos/{memo}/reactions`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `body`
    pub async fn memo_service_upsert_memo_reaction<'a>(
        &'a self,
        memo: &'a str,
        body: &'a types::UpsertMemoReactionRequest,
    ) -> Result<ResponseValue<types::Reaction>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/reactions",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_upsert_memo_reaction",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteMemoReaction deletes a reaction for a memo.
    ///
    ///Sends a `DELETE` request to `/api/v1/memos/{memo}/reactions/{reaction}`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `reaction`: The reaction id.
    pub async fn memo_service_delete_memo_reaction<'a>(
        &'a self,
        memo: &'a str,
        reaction: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/reactions/{}",
            self.baseurl,
            encode_path(&memo.to_string()),
            encode_path(&reaction.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "memo_service_delete_memo_reaction",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemoRelations lists relations for a memo.
    ///
    ///Sends a `GET` request to `/api/v1/memos/{memo}/relations`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `page_size`: Optional. The maximum number of relations to return.
    /// - `page_token`: Optional. A page token for pagination.
    pub async fn memo_service_list_memo_relations<'a>(
        &'a self,
        memo: &'a str,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListMemoRelationsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/relations",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_list_memo_relations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///SetMemoRelations replaces the full set of relations on a memo with the
    /// provided list (not an append). Pass the complete desired set; an empty
    /// list clears all relations. Idempotent.
    ///
    ///Sends a `PATCH` request to `/api/v1/memos/{memo}/relations`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `body`
    pub async fn memo_service_set_memo_relations<'a>(
        &'a self,
        memo: &'a str,
        body: &'a types::SetMemoRelationsRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/relations",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_set_memo_relations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemoShares lists all share links for a memo. Requires authentication
    /// as the memo creator.
    ///
    ///Sends a `GET` request to `/api/v1/memos/{memo}/shares`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    pub async fn memo_service_list_memo_shares<'a>(
        &'a self,
        memo: &'a str,
    ) -> Result<ResponseValue<types::ListMemoSharesResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/shares",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_list_memo_shares",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateMemoShare creates a share link for a memo. Requires authentication
    /// as the memo creator.
    ///
    ///Sends a `POST` request to `/api/v1/memos/{memo}/shares`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `body`
    pub async fn memo_service_create_memo_share<'a>(
        &'a self,
        memo: &'a str,
        body: &'a types::MemoShare,
    ) -> Result<ResponseValue<types::MemoShare>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/shares",
            self.baseurl,
            encode_path(&memo.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_create_memo_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteMemoShare revokes a share link. Requires authentication as the
    /// memo creator.
    ///
    ///Sends a `DELETE` request to `/api/v1/memos/{memo}/shares/{share}`
    ///
    ///Arguments:
    /// - `memo`: The memo id.
    /// - `share`: The share id.
    pub async fn memo_service_delete_memo_share<'a>(
        &'a self,
        memo: &'a str,
        share: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/memos/{}/shares/{}",
            self.baseurl,
            encode_path(&memo.to_string()),
            encode_path(&share.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "memo_service_delete_memo_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetSharedMemo resolves a share token to its memo. No authentication
    /// required. Returns NOT_FOUND if the token is invalid or expired.
    ///
    ///Sends a `GET` request to `/api/v1/shares/{shareToken}/memo`
    ///
    ///Arguments:
    /// - `share_token`: Required. The opaque bearer token extracted from the
    ///   share URL.
    pub async fn memo_service_get_shared_memo<'a>(
        &'a self,
        share_token: &'a str,
    ) -> Result<ResponseValue<types::Memo>, Error<()>> {
        let url = format!(
            "{}/api/v1/shares/{}/memo",
            self.baseurl,
            encode_path(&share_token.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_service_get_shared_memo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListUsers returns a list of users.
    ///
    ///Sends a `GET` request to `/api/v1/users`
    ///
    ///Arguments:
    /// - `filter`: Optional. Filter to apply to the list results.
    /// Example: "username == 'steven'"
    /// Supported operators: ==
    /// Supported fields: username
    /// - `page_size`: Optional. The maximum number of users to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 users will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    /// - `page_token`: Optional. A page token, received from a previous
    ///   `ListUsers` call.
    /// Provide this to retrieve the subsequent page.
    /// - `show_deleted`: Optional. If true, show deleted users in the response.
    pub async fn user_service_list_users<'a>(
        &'a self,
        filter: Option<&'a str>,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
        show_deleted: Option<bool>,
    ) -> Result<ResponseValue<types::ListUsersResponse>, Error<()>> {
        let url = format!("{}/api/v1/users", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "showDeleted",
                &show_deleted,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_users",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateUser creates a new user.
    ///
    ///Sends a `POST` request to `/api/v1/users`
    ///
    ///Arguments:
    /// - `request_id`: Optional. An idempotency token that can be used to
    ///   ensure that multiple
    /// requests to create a user have the same result.
    /// - `user_id`: Optional. The resource ID to use for this user. If set, it
    ///   must equal
    /// user.username and follow the username format.
    /// Format: ^[a-zA-Z0-9]([a-zA-Z0-9-]{0,34}[a-zA-Z0-9])?$
    /// - `validate_only`: Optional. If set, validate the request but don't
    ///   actually create the user.
    /// - `body`
    pub async fn user_service_create_user<'a>(
        &'a self,
        request_id: Option<&'a str>,
        user_id: Option<&'a str>,
        validate_only: Option<bool>,
        body: &'a types::User,
    ) -> Result<ResponseValue<types::User>, Error<()>> {
        let url = format!("{}/api/v1/users", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "requestId",
                &request_id,
            ))
            .query(&progenitor_client::QueryParam::new("userId", &user_id))
            .query(&progenitor_client::QueryParam::new(
                "validateOnly",
                &validate_only,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_create_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetUser gets a user by username.
    /// Format: users/{user} (e.g., users/steven)
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `read_mask`: Optional. The fields to return in the response.
    /// If not specified, all fields are returned.
    pub async fn user_service_get_user<'a>(
        &'a self,
        user: &'a str,
        read_mask: Option<&'a str>,
    ) -> Result<ResponseValue<types::User>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("readMask", &read_mask))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_get_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteUser deletes a user.
    ///
    ///Sends a `DELETE` request to `/api/v1/users/{user}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `force`: Optional. If set to true, the user will be deleted even if
    ///   they have associated data.
    pub async fn user_service_delete_user<'a>(
        &'a self,
        user: &'a str,
        force: Option<bool>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .query(&progenitor_client::QueryParam::new("force", &force))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_delete_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateUser updates a user.
    ///
    ///Sends a `PATCH` request to `/api/v1/users/{user}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `allow_missing`: Optional. If set to true, allows updating sensitive
    ///   fields.
    /// - `update_mask`: Required. The list of fields to update.
    /// - `body`
    pub async fn user_service_update_user<'a>(
        &'a self,
        user: &'a str,
        allow_missing: Option<bool>,
        update_mask: Option<&'a str>,
        body: &'a types::User,
    ) -> Result<ResponseValue<types::User>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "allowMissing",
                &allow_missing,
            ))
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_update_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListLinkedIdentities returns a list of linked SSO identities for a user.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/linkedIdentities`
    ///
    ///Arguments:
    /// - `user`: The user id.
    pub async fn user_service_list_linked_identities<'a>(
        &'a self,
        user: &'a str,
    ) -> Result<ResponseValue<types::ListLinkedIdentitiesResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/linkedIdentities",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_linked_identities",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateLinkedIdentity links an SSO identity to the authenticated user.
    ///
    ///Sends a `POST` request to `/api/v1/users/{user}/linkedIdentities`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `body`
    pub async fn user_service_create_linked_identity<'a>(
        &'a self,
        user: &'a str,
        body: &'a types::CreateLinkedIdentityRequest,
    ) -> Result<ResponseValue<types::LinkedIdentity>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/linkedIdentities",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_create_linked_identity",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetLinkedIdentity gets a linked SSO identity for a user.
    ///
    ///Sends a `GET` request to
    /// `/api/v1/users/{user}/linkedIdentities/{linkedIdentity}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `linked_identity`: The linkedIdentity id.
    pub async fn user_service_get_linked_identity<'a>(
        &'a self,
        user: &'a str,
        linked_identity: &'a str,
    ) -> Result<ResponseValue<types::LinkedIdentity>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/linkedIdentities/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&linked_identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_get_linked_identity",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteLinkedIdentity unlinks an SSO identity from a user.
    ///
    ///Sends a `DELETE` request to
    /// `/api/v1/users/{user}/linkedIdentities/{linkedIdentity}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `linked_identity`: The linkedIdentity id.
    pub async fn user_service_delete_linked_identity<'a>(
        &'a self,
        user: &'a str,
        linked_identity: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/linkedIdentities/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&linked_identity.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "user_service_delete_linked_identity",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListUserNotifications lists notifications for a user.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/notifications`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `filter`
    /// - `page_size`
    /// - `page_token`
    pub async fn user_service_list_user_notifications<'a>(
        &'a self,
        user: &'a str,
        filter: Option<&'a str>,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListUserNotificationsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/notifications",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_user_notifications",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteUserNotification deletes a notification.
    ///
    ///Sends a `DELETE` request to
    /// `/api/v1/users/{user}/notifications/{notification}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `notification`: The notification id.
    pub async fn user_service_delete_user_notification<'a>(
        &'a self,
        user: &'a str,
        notification: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/notifications/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&notification.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "user_service_delete_user_notification",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateUserNotification updates a notification.
    ///
    ///Sends a `PATCH` request to
    /// `/api/v1/users/{user}/notifications/{notification}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `notification`: The notification id.
    /// - `update_mask`
    /// - `body`
    pub async fn user_service_update_user_notification<'a>(
        &'a self,
        user: &'a str,
        notification: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::UserNotification,
    ) -> Result<ResponseValue<types::UserNotification>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/notifications/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&notification.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_update_user_notification",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListPersonalAccessTokens returns a list of Personal Access Tokens (PATs)
    /// for a user. PATs are long-lived tokens for API/script access,
    /// distinct from short-lived JWT access tokens.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/personalAccessTokens`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `page_size`: Optional. The maximum number of tokens to return.
    /// - `page_token`: Optional. A page token for pagination.
    pub async fn user_service_list_personal_access_tokens<'a>(
        &'a self,
        user: &'a str,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListPersonalAccessTokensResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/personalAccessTokens",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_personal_access_tokens",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreatePersonalAccessToken creates a new Personal Access Token for a
    /// user. The token value is only returned once upon creation.
    ///
    ///Sends a `POST` request to `/api/v1/users/{user}/personalAccessTokens`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `body`
    pub async fn user_service_create_personal_access_token<'a>(
        &'a self,
        user: &'a str,
        body: &'a types::CreatePersonalAccessTokenRequest,
    ) -> Result<ResponseValue<types::CreatePersonalAccessTokenResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/personalAccessTokens",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_create_personal_access_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeletePersonalAccessToken deletes a Personal Access Token.
    ///
    ///Sends a `DELETE` request to
    /// `/api/v1/users/{user}/personalAccessTokens/{personalAccessToken}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `personal_access_token`: The personalAccessToken id.
    pub async fn user_service_delete_personal_access_token<'a>(
        &'a self,
        user: &'a str,
        personal_access_token: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/personalAccessTokens/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&personal_access_token.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "user_service_delete_personal_access_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListUserSettings returns a list of user settings.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/settings`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `page_size`: Optional. The maximum number of settings to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 settings will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    /// - `page_token`: Optional. A page token, received from a previous
    ///   `ListUserSettings` call.
    /// Provide this to retrieve the subsequent page.
    pub async fn user_service_list_user_settings<'a>(
        &'a self,
        user: &'a str,
        page_size: Option<i32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListUserSettingsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/settings",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("pageSize", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "pageToken",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_user_settings",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetUserSetting returns the user setting.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/settings/{setting}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `setting`: The setting id.
    pub async fn user_service_get_user_setting<'a>(
        &'a self,
        user: &'a str,
        setting: &'a str,
    ) -> Result<ResponseValue<types::UserSetting>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/settings/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&setting.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_get_user_setting",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateUserSetting updates the user setting.
    ///
    ///Sends a `PATCH` request to `/api/v1/users/{user}/settings/{setting}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `setting`: The setting id.
    /// - `update_mask`: Required. The list of fields to update.
    /// - `body`
    pub async fn user_service_update_user_setting<'a>(
        &'a self,
        user: &'a str,
        setting: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::UserSetting,
    ) -> Result<ResponseValue<types::UserSetting>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/settings/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&setting.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_update_user_setting",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListMemoViews returns a user's memo views. Each view is a named,
    /// reusable CEL filter (see MemoView.filter); pass its filter string
    /// directly to the ListMemos `filter` argument.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/views`
    ///
    ///Arguments:
    /// - `user`: The user id.
    pub async fn memo_view_service_list_memo_views<'a>(
        &'a self,
        user: &'a str,
    ) -> Result<ResponseValue<types::ListMemoViewsResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/views",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_view_service_list_memo_views",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateMemoView creates a new memo view for a user.
    ///
    ///Sends a `POST` request to `/api/v1/users/{user}/views`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `validate_only`: Optional. If set, validate the request, but do not
    ///   actually create the memo view.
    /// - `body`
    pub async fn memo_view_service_create_memo_view<'a>(
        &'a self,
        user: &'a str,
        validate_only: Option<bool>,
        body: &'a types::MemoView,
    ) -> Result<ResponseValue<types::MemoView>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/views",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "validateOnly",
                &validate_only,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_view_service_create_memo_view",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetMemoView gets a memo view by name.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/views/{view}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `view`: The view id.
    pub async fn memo_view_service_get_memo_view<'a>(
        &'a self,
        user: &'a str,
        view: &'a str,
    ) -> Result<ResponseValue<types::MemoView>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/views/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&view.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_view_service_get_memo_view",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteMemoView deletes a memo view for a user.
    ///
    ///Sends a `DELETE` request to `/api/v1/users/{user}/views/{view}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `view`: The view id.
    pub async fn memo_view_service_delete_memo_view<'a>(
        &'a self,
        user: &'a str,
        view: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/views/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&view.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "memo_view_service_delete_memo_view",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateMemoView updates a memo view for a user.
    ///
    ///Sends a `PATCH` request to `/api/v1/users/{user}/views/{view}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `view`: The view id.
    /// - `update_mask`: Optional. The list of fields to update.
    /// - `body`
    pub async fn memo_view_service_update_memo_view<'a>(
        &'a self,
        user: &'a str,
        view: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::MemoView,
    ) -> Result<ResponseValue<types::MemoView>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/views/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&view.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "memo_view_service_update_memo_view",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListUserWebhooks returns a list of webhooks for a user.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}/webhooks`
    ///
    ///Arguments:
    /// - `user`: The user id.
    pub async fn user_service_list_user_webhooks<'a>(
        &'a self,
        user: &'a str,
    ) -> Result<ResponseValue<types::ListUserWebhooksResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/webhooks",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_user_webhooks",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///CreateUserWebhook creates a new webhook for a user.
    ///
    ///Sends a `POST` request to `/api/v1/users/{user}/webhooks`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `body`
    pub async fn user_service_create_user_webhook<'a>(
        &'a self,
        user: &'a str,
        body: &'a types::UserWebhook,
    ) -> Result<ResponseValue<types::UserWebhook>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/webhooks",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_create_user_webhook",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///DeleteUserWebhook deletes a webhook for a user.
    ///
    ///Sends a `DELETE` request to `/api/v1/users/{user}/webhooks/{webhook}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `webhook`: The webhook id.
    pub async fn user_service_delete_user_webhook<'a>(
        &'a self,
        user: &'a str,
        webhook: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/webhooks/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&webhook.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "user_service_delete_user_webhook",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///UpdateUserWebhook updates an existing webhook for a user.
    ///
    ///Sends a `PATCH` request to `/api/v1/users/{user}/webhooks/{webhook}`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `webhook`: The webhook id.
    /// - `update_mask`: The list of fields to update.
    /// - `body`
    pub async fn user_service_update_user_webhook<'a>(
        &'a self,
        user: &'a str,
        webhook: &'a str,
        update_mask: Option<&'a str>,
        body: &'a types::UserWebhook,
    ) -> Result<ResponseValue<types::UserWebhook>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/webhooks/{}",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&webhook.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new(
                "updateMask",
                &update_mask,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_update_user_webhook",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetUserWebhookSigningSecret returns the signing secret for a webhook.
    /// The secret is returned only through this explicit, owner-gated call; it
    /// is never included in List/Create/Update responses.
    ///
    ///Sends a `GET` request to
    /// `/api/v1/users/{user}/webhooks/{webhook}:getSigningSecret`
    ///
    ///Arguments:
    /// - `user`: The user id.
    /// - `webhook`: The webhook id.
    pub async fn user_service_get_user_webhook_signing_secret<'a>(
        &'a self,
        user: &'a str,
        webhook: &'a str,
    ) -> Result<ResponseValue<types::GetUserWebhookSigningSecretResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/webhooks/{}:getSigningSecret",
            self.baseurl,
            encode_path(&user.to_string()),
            encode_path(&webhook.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_get_user_webhook_signing_secret",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///GetUserStats returns statistics for a specific user.
    ///
    ///Sends a `GET` request to `/api/v1/users/{user}:getStats`
    ///
    ///Arguments:
    /// - `user`: The user id.
    pub async fn user_service_get_user_stats<'a>(
        &'a self,
        user: &'a str,
    ) -> Result<ResponseValue<types::UserStats>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}:getStats",
            self.baseurl,
            encode_path(&user.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_get_user_stats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///BatchGetUsers returns active users by usernames.
    ///
    ///Sends a `POST` request to `/api/v1/users:batchGet`
    pub async fn user_service_batch_get_users<'a>(
        &'a self,
        body: &'a types::BatchGetUsersRequest,
    ) -> Result<ResponseValue<types::BatchGetUsersResponse>, Error<()>> {
        let url = format!("{}/api/v1/users:batchGet", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_batch_get_users",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///ListAllUserStats returns statistics for all users.
    ///
    ///Sends a `GET` request to `/api/v1/users:stats`
    ///
    ///Arguments:
    /// - `filter`: Optional. Filter to apply to memo stats.
    /// Uses the same filter syntax as ListMemos.
    /// - `state`: Optional. The state of memos to include. Defaults to NORMAL.
    pub async fn user_service_list_all_user_stats<'a>(
        &'a self,
        filter: Option<&'a str>,
        state: Option<types::UserServiceListAllUserStatsState>,
    ) -> Result<ResponseValue<types::ListAllUserStatsResponse>, Error<()>> {
        let url = format!("{}/api/v1/users:stats", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("state", &state))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_service_list_all_user_stats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}

use std::ffi::{c_char, CStr};

use openxr::{raw, ExtensionSet};

pub fn ext_set_from_names(names: &[&CStr]) -> ExtensionSet {
    let mut out = ExtensionSet::default();
    for ext in names.iter() {
        match ext.to_bytes_with_nul() {
            raw::DigitalLensControlALMALENCE::NAME => {
                out.almalence_digital_lens_control = true;
            }
            raw::ControllerInteractionBD::NAME => {
                out.bd_controller_interaction = true;
            }
            raw::ViewConfigurationFovEPIC::NAME => {
                out.epic_view_configuration_fov = true;
            }
            raw::PerformanceSettingsEXT::NAME => {
                out.ext_performance_settings = true;
            }
            raw::ThermalQueryEXT::NAME => {
                out.ext_thermal_query = true;
            }
            raw::DebugUtilsEXT::NAME => {
                out.ext_debug_utils = true;
            }
            raw::EyeGazeInteractionEXT::NAME => {
                out.ext_eye_gaze_interaction = true;
            }
            raw::ViewConfigurationDepthRangeEXT::NAME => {
                out.ext_view_configuration_depth_range = true;
            }
            raw::ConformanceAutomationEXT::NAME => {
                out.ext_conformance_automation = true;
            }
            raw::HandTrackingEXT::NAME => {
                out.ext_hand_tracking = true;
            }
            #[cfg(windows)]
            raw::Win32AppcontainerCompatibleEXT::NAME => {
                out.ext_win32_appcontainer_compatible = true;
            }
            raw::DpadBindingEXT::NAME => {
                out.ext_dpad_binding = true;
            }
            raw::HandJointsMotionRangeEXT::NAME => {
                out.ext_hand_joints_motion_range = true;
            }
            raw::SamsungOdysseyControllerEXT::NAME => {
                out.ext_samsung_odyssey_controller = true;
            }
            raw::HpMixedRealityControllerEXT::NAME => {
                out.ext_hp_mixed_reality_controller = true;
            }
            raw::PalmPoseEXT::NAME => {
                out.ext_palm_pose = true;
            }
            raw::UuidEXT::NAME => {
                out.ext_uuid = true;
            }
            raw::HandInteractionEXT::NAME => {
                out.ext_hand_interaction = true;
            }
            raw::ActiveActionSetPriorityEXT::NAME => {
                out.ext_active_action_set_priority = true;
            }
            raw::LocalFloorEXT::NAME => {
                out.ext_local_floor = true;
            }
            raw::HandTrackingDataSourceEXT::NAME => {
                out.ext_hand_tracking_data_source = true;
            }
            raw::PlaneDetectionEXT::NAME => {
                out.ext_plane_detection = true;
            }
            raw::FutureEXT::NAME => {
                out.ext_future = true;
            }
            raw::UserPresenceEXT::NAME => {
                out.ext_user_presence = true;
            }
            raw::CompositionLayerImageLayoutFB::NAME => {
                out.fb_composition_layer_image_layout = true;
            }
            raw::CompositionLayerAlphaBlendFB::NAME => {
                out.fb_composition_layer_alpha_blend = true;
            }
            #[cfg(target_os = "android")]
            raw::AndroidSurfaceSwapchainCreateFB::NAME => {
                out.fb_android_surface_swapchain_create = true;
            }
            raw::SwapchainUpdateStateFB::NAME => {
                out.fb_swapchain_update_state = true;
            }
            raw::CompositionLayerSecureContentFB::NAME => {
                out.fb_composition_layer_secure_content = true;
            }
            raw::BodyTrackingFB::NAME => {
                out.fb_body_tracking = true;
            }
            raw::DisplayRefreshRateFB::NAME => {
                out.fb_display_refresh_rate = true;
            }
            raw::ColorSpaceFB::NAME => {
                out.fb_color_space = true;
            }
            raw::HandTrackingMeshFB::NAME => {
                out.fb_hand_tracking_mesh = true;
            }
            raw::HandTrackingAimFB::NAME => {
                out.fb_hand_tracking_aim = true;
            }
            raw::HandTrackingCapsulesFB::NAME => {
                out.fb_hand_tracking_capsules = true;
            }
            raw::SpatialEntityFB::NAME => {
                out.fb_spatial_entity = true;
            }
            raw::FoveationFB::NAME => {
                out.fb_foveation = true;
            }
            raw::FoveationConfigurationFB::NAME => {
                out.fb_foveation_configuration = true;
            }
            raw::KeyboardTrackingFB::NAME => {
                out.fb_keyboard_tracking = true;
            }
            raw::TriangleMeshFB::NAME => {
                out.fb_triangle_mesh = true;
            }
            raw::PassthroughFB::NAME => {
                out.fb_passthrough = true;
            }
            raw::RenderModelFB::NAME => {
                out.fb_render_model = true;
            }
            raw::SpatialEntityQueryFB::NAME => {
                out.fb_spatial_entity_query = true;
            }
            raw::SpatialEntityStorageFB::NAME => {
                out.fb_spatial_entity_storage = true;
            }
            raw::FoveationVulkanFB::NAME => {
                out.fb_foveation_vulkan = true;
            }
            #[cfg(target_os = "android")]
            raw::SwapchainUpdateStateAndroidSurfaceFB::NAME => {
                out.fb_swapchain_update_state_android_surface = true;
            }
            raw::SwapchainUpdateStateOpenglEsFB::NAME => {
                out.fb_swapchain_update_state_opengl_es = true;
            }
            raw::SwapchainUpdateStateVulkanFB::NAME => {
                out.fb_swapchain_update_state_vulkan = true;
            }
            raw::TouchControllerProFB::NAME => {
                out.fb_touch_controller_pro = true;
            }
            raw::SpatialEntitySharingFB::NAME => {
                out.fb_spatial_entity_sharing = true;
            }
            raw::SpaceWarpFB::NAME => {
                out.fb_space_warp = true;
            }
            raw::HapticAmplitudeEnvelopeFB::NAME => {
                out.fb_haptic_amplitude_envelope = true;
            }
            raw::SceneFB::NAME => {
                out.fb_scene = true;
            }
            raw::SceneCaptureFB::NAME => {
                out.fb_scene_capture = true;
            }
            raw::SpatialEntityContainerFB::NAME => {
                out.fb_spatial_entity_container = true;
            }
            raw::FaceTrackingFB::NAME => {
                out.fb_face_tracking = true;
            }
            raw::EyeTrackingSocialFB::NAME => {
                out.fb_eye_tracking_social = true;
            }
            raw::PassthroughKeyboardHandsFB::NAME => {
                out.fb_passthrough_keyboard_hands = true;
            }
            raw::CompositionLayerSettingsFB::NAME => {
                out.fb_composition_layer_settings = true;
            }
            raw::TouchControllerProximityFB::NAME => {
                out.fb_touch_controller_proximity = true;
            }
            raw::HapticPcmFB::NAME => {
                out.fb_haptic_pcm = true;
            }
            raw::CompositionLayerDepthTestFB::NAME => {
                out.fb_composition_layer_depth_test = true;
            }
            raw::SpatialEntityStorageBatchFB::NAME => {
                out.fb_spatial_entity_storage_batch = true;
            }
            raw::SpatialEntityUserFB::NAME => {
                out.fb_spatial_entity_user = true;
            }
            raw::FaceTracking2FB::NAME => {
                out.fb_face_tracking2 = true;
            }
            raw::ViveCosmosControllerInteractionHTC::NAME => {
                out.htc_vive_cosmos_controller_interaction = true;
            }
            raw::FacialTrackingHTC::NAME => {
                out.htc_facial_tracking = true;
            }
            raw::ViveFocus3ControllerInteractionHTC::NAME => {
                out.htc_vive_focus3_controller_interaction = true;
            }
            raw::HandInteractionHTC::NAME => {
                out.htc_hand_interaction = true;
            }
            raw::ViveWristTrackerInteractionHTC::NAME => {
                out.htc_vive_wrist_tracker_interaction = true;
            }
            raw::PassthroughHTC::NAME => {
                out.htc_passthrough = true;
            }
            raw::FoveationHTC::NAME => {
                out.htc_foveation = true;
            }
            raw::AnchorHTC::NAME => {
                out.htc_anchor = true;
            }
            raw::ControllerInteractionHUAWEI::NAME => {
                out.huawei_controller_interaction = true;
            }
            #[cfg(target_os = "android")]
            raw::AndroidThreadSettingsKHR::NAME => {
                out.khr_android_thread_settings = true;
            }
            #[cfg(target_os = "android")]
            raw::AndroidSurfaceSwapchainKHR::NAME => {
                out.khr_android_surface_swapchain = true;
            }
            raw::CompositionLayerCubeKHR::NAME => {
                out.khr_composition_layer_cube = true;
            }
            #[cfg(target_os = "android")]
            raw::AndroidCreateInstanceKHR::NAME => {
                out.khr_android_create_instance = true;
            }
            raw::CompositionLayerDepthKHR::NAME => {
                out.khr_composition_layer_depth = true;
            }
            raw::VulkanSwapchainFormatListKHR::NAME => {
                out.khr_vulkan_swapchain_format_list = true;
            }
            raw::CompositionLayerCylinderKHR::NAME => {
                out.khr_composition_layer_cylinder = true;
            }
            raw::CompositionLayerEquirectKHR::NAME => {
                out.khr_composition_layer_equirect = true;
            }
            raw::OpenglEnableKHR::NAME => {
                out.khr_opengl_enable = true;
            }
            raw::OpenglEsEnableKHR::NAME => {
                out.khr_opengl_es_enable = true;
            }
            raw::VulkanEnableKHR::NAME => {
                out.khr_vulkan_enable = true;
            }
            #[cfg(windows)]
            raw::D3d11EnableKHR::NAME => {
                out.khr_d3d11_enable = true;
            }
            #[cfg(windows)]
            raw::D3d12EnableKHR::NAME => {
                out.khr_d3d12_enable = true;
            }
            raw::VisibilityMaskKHR::NAME => {
                out.khr_visibility_mask = true;
            }
            raw::CompositionLayerColorScaleBiasKHR::NAME => {
                out.khr_composition_layer_color_scale_bias = true;
            }
            #[cfg(windows)]
            raw::Win32ConvertPerformanceCounterTimeKHR::NAME => {
                out.khr_win32_convert_performance_counter_time = true;
            }
            raw::ConvertTimespecTimeKHR::NAME => {
                out.khr_convert_timespec_time = true;
            }
            raw::LoaderInitKHR::NAME => {
                out.khr_loader_init = true;
            }
            #[cfg(target_os = "android")]
            raw::LoaderInitAndroidKHR::NAME => {
                out.khr_loader_init_android = true;
            }
            raw::VulkanEnable2KHR::NAME => {
                out.khr_vulkan_enable2 = true;
            }
            raw::CompositionLayerEquirect2KHR::NAME => {
                out.khr_composition_layer_equirect2 = true;
            }
            raw::BindingModificationKHR::NAME => {
                out.khr_binding_modification = true;
            }
            raw::SwapchainUsageInputAttachmentBitKHR::NAME => {
                out.khr_swapchain_usage_input_attachment_bit = true;
            }
            raw::LocateSpacesKHR::NAME => {
                out.khr_locate_spaces = true;
            }
            raw::Maintenance1KHR::NAME => {
                out.khr_maintenance1 = true;
            }
            raw::FoveationEyeTrackedMETA::NAME => {
                out.meta_foveation_eye_tracked = true;
            }
            raw::LocalDimmingMETA::NAME => {
                out.meta_local_dimming = true;
            }
            raw::PassthroughPreferencesMETA::NAME => {
                out.meta_passthrough_preferences = true;
            }
            raw::VirtualKeyboardMETA::NAME => {
                out.meta_virtual_keyboard = true;
            }
            raw::VulkanSwapchainCreateInfoMETA::NAME => {
                out.meta_vulkan_swapchain_create_info = true;
            }
            raw::PerformanceMetricsMETA::NAME => {
                out.meta_performance_metrics = true;
            }
            raw::HeadsetIdMETA::NAME => {
                out.meta_headset_id = true;
            }
            raw::RecommendedLayerResolutionMETA::NAME => {
                out.meta_recommended_layer_resolution = true;
            }
            raw::PassthroughColorLutMETA::NAME => {
                out.meta_passthrough_color_lut = true;
            }
            raw::SpatialEntityMeshMETA::NAME => {
                out.meta_spatial_entity_mesh = true;
            }
            raw::AutomaticLayerFilterMETA::NAME => {
                out.meta_automatic_layer_filter = true;
            }
            raw::TouchControllerPlusMETA::NAME => {
                out.meta_touch_controller_plus = true;
            }
            raw::EnvironmentDepthMETA::NAME => {
                out.meta_environment_depth = true;
            }
            raw::Ml2ControllerInteractionML::NAME => {
                out.ml_ml2_controller_interaction = true;
            }
            raw::FrameEndInfoML::NAME => {
                out.ml_frame_end_info = true;
            }
            raw::GlobalDimmerML::NAME => {
                out.ml_global_dimmer = true;
            }
            raw::CompatML::NAME => {
                out.ml_compat = true;
            }
            raw::MarkerUnderstandingML::NAME => {
                out.ml_marker_understanding = true;
            }
            raw::LocalizationMapML::NAME => {
                out.ml_localization_map = true;
            }
            raw::UserCalibrationML::NAME => {
                out.ml_user_calibration = true;
            }
            raw::HeadlessMND::NAME => {
                out.mnd_headless = true;
            }
            raw::SwapchainUsageInputAttachmentBitMND::NAME => {
                out.mnd_swapchain_usage_input_attachment_bit = true;
            }
            raw::UnboundedReferenceSpaceMSFT::NAME => {
                out.msft_unbounded_reference_space = true;
            }
            raw::SpatialAnchorMSFT::NAME => {
                out.msft_spatial_anchor = true;
            }
            raw::SpatialGraphBridgeMSFT::NAME => {
                out.msft_spatial_graph_bridge = true;
            }
            raw::HandInteractionMSFT::NAME => {
                out.msft_hand_interaction = true;
            }
            raw::HandTrackingMeshMSFT::NAME => {
                out.msft_hand_tracking_mesh = true;
            }
            raw::SecondaryViewConfigurationMSFT::NAME => {
                out.msft_secondary_view_configuration = true;
            }
            raw::FirstPersonObserverMSFT::NAME => {
                out.msft_first_person_observer = true;
            }
            raw::ControllerModelMSFT::NAME => {
                out.msft_controller_model = true;
            }
            #[cfg(windows)]
            raw::PerceptionAnchorInteropMSFT::NAME => {
                out.msft_perception_anchor_interop = true;
            }
            #[cfg(windows)]
            raw::HolographicWindowAttachmentMSFT::NAME => {
                out.msft_holographic_window_attachment = true;
            }
            raw::CompositionLayerReprojectionMSFT::NAME => {
                out.msft_composition_layer_reprojection = true;
            }
            raw::SpatialAnchorPersistenceMSFT::NAME => {
                out.msft_spatial_anchor_persistence = true;
            }
            #[cfg(target_os = "android")]
            raw::AndroidSessionStateEnableOCULUS::NAME => {
                out.oculus_android_session_state_enable = true;
            }
            raw::AudioDeviceGuidOCULUS::NAME => {
                out.oculus_audio_device_guid = true;
            }
            raw::ExternalCameraOCULUS::NAME => {
                out.oculus_external_camera = true;
            }
            raw::ControllerInteractionOPPO::NAME => {
                out.oppo_controller_interaction = true;
            }
            raw::TrackingOptimizationSettingsQCOM::NAME => {
                out.qcom_tracking_optimization_settings = true;
            }
            raw::HandTrackingForearmULTRALEAP::NAME => {
                out.ultraleap_hand_tracking_forearm = true;
            }
            raw::AnalogThresholdVALVE::NAME => {
                out.valve_analog_threshold = true;
            }
            raw::QuadViewsVARJO::NAME => {
                out.varjo_quad_views = true;
            }
            raw::FoveatedRenderingVARJO::NAME => {
                out.varjo_foveated_rendering = true;
            }
            raw::CompositionLayerDepthTestVARJO::NAME => {
                out.varjo_composition_layer_depth_test = true;
            }
            raw::EnvironmentDepthEstimationVARJO::NAME => {
                out.varjo_environment_depth_estimation = true;
            }
            raw::MarkerTrackingVARJO::NAME => {
                out.varjo_marker_tracking = true;
            }
            raw::ViewOffsetVARJO::NAME => {
                out.varjo_view_offset = true;
            }
            raw::Xr4ControllerInteractionVARJO::NAME => {
                out.varjo_xr4_controller_interaction = true;
            }
            raw::ControllerInteractionYVR::NAME => {
                out.yvr_controller_interaction = true;
            }
            raw::OverlayEXTX::NAME => {
                out.extx_overlay = true;
            }
            raw::EglEnableMNDX::NAME => {
                out.mndx_egl_enable = true;
            }
            raw::ForceFeedbackCurlMNDX::NAME => {
                out.mndx_force_feedback_curl = true;
            }
            raw::ViveTrackerInteractionHTCX::NAME => {
                out.htcx_vive_tracker_interaction = true;
            }
            bytes => {
                let cstr = CStr::from_bytes_with_nul(bytes)
                    .expect("extension names should be null terminated strings");
                let string = cstr
                    .to_str()
                    .expect("extension names should be valid UTF-8")
                    .to_string();
                out.other.push(string);
            }
        }
    }
    out
}

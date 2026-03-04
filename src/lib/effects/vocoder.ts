enum VocoderParameter {
  BAND_COUNT = 'band_count',
  MIN_FREQ = 'min_freq',
  MAX_FREQ = 'max_freq',
  Q = 'q',
  ATTACK_MS = 'attack_ms',
  RELEASE_MS = 'release_ms',
  OUTPUT_GAIN = 'output_gain',
  DRY_MIX = 'dry_mix',
  ENV_GAIN = 'env_gain',
  ENV_FLOOR = 'env_floor',
  SOFT_CLIP = 'soft_clip',
  REVERB_MIX = 'reverb_mix',
  CARRIER_BASE_FREQ = 'carrier_base_freq',
  CARRIER_HARMONICS = 'carrier_harmonics',
  CARRIER_GAIN = 'carrier_gain',
}

export type VocoderSettings = {
  parameters: Record<VocoderParameter, number>,
}

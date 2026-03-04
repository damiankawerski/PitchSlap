enum DistortionParameter {
  GAIN = 'gain',
}

export type DistortionSettings = {
  parameters: Record<DistortionParameter, number>,
}

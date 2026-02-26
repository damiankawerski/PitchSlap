enum Scale {
    CMajor,
    AMajor,
    GMajor,
    DMajor,
    EMajor,
    FMajor,
    GMinor,
    DMinor,
    AMinor,
    EMinor,
}

enum AutoTuneParameter {
  CORRECTION_SPEED = 'correction_speed',
  DETECTION_WINDOW_SIZE = 'detection_window_size',
  POWER_THRESHOLD = 'power_threshold',
  CLARITY_THRESHOLD = 'clarity_threshold',
}

export type AutoTuneSettings = {
  scale: Scale,
  parameters: Record<AutoTuneParameter, number>,
}
enum ReverbParameter {
  ROOM_SIZE = 'room_size',
  DAMPING = 'damping',
  WET_LEVEL = 'wet_level',
  DRY_LEVEL = 'dry_level',
  WIDTH = 'width',
}

export type ReverbSettings = {
  parameters: Record<ReverbParameter, number>,
}

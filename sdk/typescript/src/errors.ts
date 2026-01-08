export class KombtError extends Error {
  constructor(public code: string, message: string) {
    super(message);
    this.name = "KombtError";
  }

  static invalidStake(): KombtError {
    return new KombtError("InvalidStake", "stake amount must be positive");
  }

  static expired(): KombtError {
    return new KombtError("Expired", "match has expired");
  }

  static unauthorized(): KombtError {
    return new KombtError("Unauthorized", "signer is not authorized");
  }
}

// rev3

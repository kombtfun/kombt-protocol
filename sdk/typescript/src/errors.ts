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

// rev7

// rev24

// rev43

// rev47

// rev76

// rev81

// rev112

// rev121

// rev171

// rev184

// rev192

// rev206

// rev210

// rev230

// rev236

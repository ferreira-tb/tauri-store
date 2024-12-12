import { parseBigInt } from './index';
import type { Option } from '@tb-dev/utils';

export type TimeStrategyKind = 'debounce' | 'throttle' | 'immediate';

export type LooseTimeStrategyKind = Option<TimeStrategyKind | number>;

/** @internal */
export type TimeStrategyRawTuple = [TimeStrategyKind, string];

export class TimeStrategy {
  public readonly interval: number;
  public readonly strategy: TimeStrategyKind;

  constructor(strategy: LooseTimeStrategyKind, interval?: Option<number>) {
    this.strategy = toStrategyKind(strategy);
    this.interval = isValidInterval(interval) ? interval : 0;

    if (!this.interval) {
      if (isValidInterval(strategy)) {
        this.interval = strategy;
      } else {
        this.strategy = 'immediate';
      }
    }
  }

  /** @internal */
  public tuple(): TimeStrategyRawTuple {
    return [this.strategy, this.interval.toString(10)];
  }

  /**
   * Rust represents milliseconds as an unsigned 128-bit integer,
   * which is too large to be represented safely in JavaScript.
   *
   * For this reason, it must return the value as a string.
   *
   * @internal
   */
  public static parse(tuple: TimeStrategyRawTuple) {
    return new TimeStrategy(tuple[0], parseBigInt(tuple[1]));
  }
}

function toStrategyKind(strategy: LooseTimeStrategyKind): TimeStrategyKind {
  if (typeof strategy === 'string') {
    return strategy;
  } else if (isValidInterval(strategy)) {
    return 'debounce';
  }

  return 'immediate';
}

function isValidInterval(interval: unknown): interval is number {
  return typeof interval === 'number' && Number.isFinite(interval) && interval > 0;
}

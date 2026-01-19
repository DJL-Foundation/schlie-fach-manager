/**
 * Screensaver Animations
 * @module screensaver/animations
 */

import type { Animation } from '../types/ui';

/**
 * Animation player for managing animation frames
 */
export class AnimationPlayer {
  private currentFrame = 0;
  private lastFrameTime = Date.now();

  constructor(private animation: Animation) {}

  /**
   * Update animation frame if needed
   */
  update(): void {
    const elapsed = Date.now() - this.lastFrameTime;

    if (elapsed >= this.animation.frameDelayMs) {
      this.currentFrame = (this.currentFrame + 1) % this.animation.frames.length;
      this.lastFrameTime = Date.now();
    }
  }

  /**
   * Get current frame content
   */
  getCurrentFrame(): string {
    return this.animation.frames[this.currentFrame];
  }

  /**
   * Get animation metadata
   */
  getAnimation(): Animation {
    return this.animation;
  }

  /**
   * Reset animation to first frame
   */
  reset(): void {
    this.currentFrame = 0;
    this.lastFrameTime = Date.now();
  }
}

/**
 * Spinning Clock Animation
 */
export const SpinningClock: Animation = {
  name: 'Spinning Clock',
  frames: ['🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘', '🕙', '🕚', '🕛'],
  frameDelayMs: 150,
  width: 2,
  height: 1,
};

/**
 * Bouncing Box Animation
 */
export const BouncingBox: Animation = {
  name: 'Bouncing Box',
  frames: [
    `┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘`,
    `┌─────────────┐
│  ┌─────┐    │
│  │  ■  │    │
│  └─────┘    │
│             │
│             │
└─────────────┘`,
    `┌─────────────┐
│┌─────┐      │
││  ■  │      │
│└─────┘      │
│             │
│             │
└─────────────┘`,
    `┌─────────────┐
│             │
│┌─────┐      │
││  ■  │      │
│└─────┘      │
│             │
└─────────────┘`,
    `┌─────────────┐
│             │
│             │
│┌─────┐      │
││  ■  │      │
│└─────┘      │
└─────────────┘`,
    `┌─────────────┐
│             │
│             │
│  ┌─────┐    │
│  │  ■  │    │
│  └─────┘    │
└─────────────┘`,
    `┌─────────────┐
│             │
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
└─────────────┘`,
    `┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘`,
  ],
  frameDelayMs: 200,
  width: 15,
  height: 7,
};

/**
 * Matrix Rain Animation
 */
export const MatrixRain: Animation = {
  name: 'Matrix Rain',
  frames: [
    `  1   0     1  
    0   1     0
  1       0    
      1       1
0       1      
    1       0  
        0   1  
  1   1        `,
    `    0   1      
  1       0    
      1        
0       1   1  
    1       0  
        0   1  
  1   1        
      0        `,
    `  1     0   1  
      1        
    0   1      
  1       1    
      0        
1       0   1  
    1          
        1   0  `,
    `      1     0  
1   0       1  
    1   0      
        1      
  1       0    
      1        
0       1   1  
  0       1    `,
  ],
  frameDelayMs: 200,
  width: 15,
  height: 8,
};

/**
 * Loading Spinner Animation
 */
export const LoadingSpinner: Animation = {
  name: 'Loading Spinner',
  frames: [
    `╔════════════╗
║ ⠋ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠙ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠹ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠸ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠼ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠴ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠦ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠧ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠇ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠏ Loading  ║
╚════════════╝`,
  ],
  frameDelayMs: 100,
  width: 14,
  height: 3,
};

/**
 * Waving Text Animation
 */
export const WavingText: Animation = {
  name: 'Waving Text',
  frames: [
    `╔═══════════════════════╗
║                       ║
║   Schließfach-Manager ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║  Schließfach-Manager  ║
║                       ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║                       ║
║  Schließfach-Manager  ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║                       ║
║                       ║
║  Schließfach-Manager  ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║                       ║
║  Schließfach-Manager  ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║  Schließfach-Manager  ║
║                       ║
║                       ║
╚═══════════════════════╝`,
  ],
  frameDelayMs: 300,
  width: 25,
  height: 5,
};

/**
 * All available animations
 */
export const ALL_ANIMATIONS: Animation[] = [
  SpinningClock,
  BouncingBox,
  MatrixRain,
  LoadingSpinner,
  WavingText,
];

/**
 * Get a random animation
 * @returns Random animation from available animations
 */
export function getRandomAnimation(): Animation {
  return ALL_ANIMATIONS[Math.floor(Math.random() * ALL_ANIMATIONS.length)];
}

/**
 * Get animation by name
 * @param name Animation name
 * @returns Animation or undefined if not found
 */
export function getAnimationByName(name: string): Animation | undefined {
  return ALL_ANIMATIONS.find((a) => a.name === name);
}

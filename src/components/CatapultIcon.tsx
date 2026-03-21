export default function CatapultIcon({
  size = 24,
  className = "",
}: {
  size?: number;
  className?: string;
}) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
    >
      {/* Base */}
      <line x1="3" y1="21" x2="21" y2="21" />
      {/* A-frame */}
      <line x1="9" y1="21" x2="12" y2="9" />
      <line x1="15" y1="21" x2="12" y2="9" />
      {/* Throwing arm */}
      <line x1="5" y1="4" x2="17" y2="13" />
      {/* Projectile */}
      <circle cx="4" cy="3" r="2" fill="currentColor" stroke="none" />
    </svg>
  );
}

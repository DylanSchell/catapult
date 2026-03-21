import { NavLink, Outlet } from "react-router-dom";
import {
  LayoutDashboard,
  Download,
  Database,
  Play,
  MessageSquare,
} from "lucide-react";
import { clsx } from "clsx";
import CatapultIcon from "./CatapultIcon";

const navItems = [
  { to: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
  { to: "/runtime", label: "Runtime", icon: Download },
  { to: "/models", label: "Models", icon: Database },
  { to: "/server", label: "Run", icon: Play },
  { to: "/chat", label: "Chat", icon: MessageSquare },
];

export default function Layout() {
  return (
    <div className="flex h-full bg-surface-0">
      {/* Sidebar */}
      <aside className="w-52 flex flex-col bg-surface-1 border-r border-border shrink-0">
        {/* Logo */}
        <div className="flex items-center gap-2.5 px-5 py-5 border-b border-border">
          <div className="w-7 h-7 bg-primary flex items-center justify-center shrink-0">
            <CatapultIcon size={14} className="text-white" />
          </div>
          <span className="font-semibold text-gray-100 text-base tracking-tight">
            Catapult
          </span>
        </div>

        {/* Nav */}
        <nav className="flex-1 py-3 px-2">
          {navItems.map(({ to, label, icon: Icon }) => (
            <NavLink
              key={to}
              to={to}
              className={({ isActive }) =>
                clsx(
                  "flex items-center gap-3 px-3 py-2.5 mb-0.5 text-sm font-medium transition-colors",
                  isActive
                    ? "bg-primary/20 text-primary-light"
                    : "text-gray-400 hover:text-gray-200 hover:bg-surface-3"
                )
              }
            >
              <Icon size={16} />
              {label}
            </NavLink>
          ))}
        </nav>

        <div className="px-4 py-3 border-t border-border">
          <p className="text-xs text-gray-600">llama.cpp launcher</p>
        </div>
      </aside>

      {/* Main */}
      <main className="flex-1 overflow-hidden flex flex-col min-w-0">
        <Outlet />
      </main>
    </div>
  );
}

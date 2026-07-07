from __future__ import annotations

from dataclasses import dataclass

from atomic_kernels._atomic_kernels import RenderConfig, ViewerConfig


QUALITY_PRESET_ICO_SUBDIV = {
    "low": 3,
    "medium": 4,
    "high": 5,
    "very_high": 7,
}

QUALITY_PRESET_ALIASES = {
    "l": "low",
    "m": "medium",
    "h": "high",
    "vh": "very_high",
}


@dataclass(frozen=True)
class QualityPreset:
    """Python-side viewer fidelity preset.

    The preset resolves to the currently supported render-fidelity knobs before the
    config is passed to Rust. Additional fidelity controls can be added here later
    without changing the Rust interface.
    """

    name: str
    ico_subdiv: int

    @classmethod
    def from_name(cls, name: str) -> QualityPreset:
        normalized = name.lower()
        normalized = QUALITY_PRESET_ALIASES.get(normalized, normalized)
        try:
            ico_subdiv = QUALITY_PRESET_ICO_SUBDIV[normalized]
        except KeyError as exc:
            supported = ", ".join(
                [
                    "low (l)",
                    "medium (m)",
                    "high (h)",
                    "very_high (vh)",
                ]
            )
            raise ValueError(
                f"unsupported quality preset {name!r}; expected one of: {supported}"
            ) from exc
        return cls(name=normalized, ico_subdiv=ico_subdiv)

    @classmethod
    def low(cls) -> QualityPreset:
        return cls.from_name("low")

    @classmethod
    def medium(cls) -> QualityPreset:
        return cls.from_name("medium")

    @classmethod
    def high(cls) -> QualityPreset:
        return cls.from_name("high")

    @classmethod
    def very_high(cls) -> QualityPreset:
        return cls.from_name("very_high")

    def apply_to_render(self, config: RenderConfig | None = None) -> RenderConfig:
        render = config if config is not None else RenderConfig()
        render.ico_subdiv = self.ico_subdiv
        return render

    def apply_to_viewer(self, config: ViewerConfig | None = None) -> ViewerConfig:
        viewer = config if config is not None else ViewerConfig()
        render = viewer.render
        render.ico_subdiv = self.ico_subdiv
        viewer.render = render
        return viewer


LOW = QualityPreset.low()
MEDIUM = QualityPreset.medium()
HIGH = QualityPreset.high()
VERY_HIGH = QualityPreset.very_high()

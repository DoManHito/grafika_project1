# Projekt 1: Implementacja wybranych algorytmów grafiki komputerowej

## I. Opis Zadania i Rozwiązania

**Temat:** Wizualizacja 3D i animacja trójkątów z wykorzystaniem kamery perspektywicznej.

### Charakterystyka rozwiązania:
* **Technologia:** Projekt zrealizowany w języku **Rust** przy użyciu API **wgpu**.
* **Kamera:** Zaimplementowano model kamery perspektywicznej o ogniskowej $f=5$. Środek rzutu znajduje się w punkcie $(0, 0, -5)$, a płaszczyzna obrazu ($640 \\\\times 480$) w płaszczyźnie OXY.
* **Część 1 (Wariant b):** Wyświetlanie statycznej sceny. Trójkąt A posiada trójkątną dziurę, co zrealizowano poprzez precyzyjne zdefiniowanie bufora indeksów (triangulacja powierzchni z wycięciem).
* **Część 2 (Wariant b):** Animacja 180 klatek. Zaimplementowano transformacje geometryczne: obrót (ROLL wokół osi OZ, PITCH wokół osi OY) oraz translację.
* **Kolorowanie:** Użyto shadera WGSL. Zaimplementowano dwustronne kolorowanie płaszczyzn przy użyciu wbudowanej zmiennej `@builtin(front_facing)`, co pozwala na uzyskanie różnych kolorów w zależności od orientacji poligonu względem kamery.
* **Model rzutowania:** Zastosowano macierz rzutowania perspektywicznego, gdzie kąt widzenia ($FOV$) został wyliczony na podstawie ogniskowej $f=5$ oraz rozmiaru piksela $0.01$.

### Pliki zrodlowe:
* **app.rs** Odpowiada za integrację z biblioteką winit (inicjalizacja okna), obsługę okna aplikacji oraz pętli zdarzeń, z którą powiązane jest renderowanie i przechwytywanie wejścia od użytkownika.
* **camera.rs** Definiuje strukturę danych kamery oraz zawiera stałe konfiguracyjne wykorzystywane do obliczeń perspektywy i mechaniki animacji (np. ognisko rzutu, mnożniki, czy stałe do rotacji A i B).
* **lib.rs** Plik łączący poszczególne moduły w spójną architekturę. Udostępnia publiczną funkcję run(), odpowiedzialną za inicjalizację głównej pętli aplikacji.
* **main.rs** Krótki punkt wejścia programu, który po prostu wywołuje główną logikę z określoną flagą wariantu zadania.
* **shader.wgsl** Kod programów kolorujacych (vertex i fragment shader) napisany w języku WGSL. Wykonuje mnożenie wierzchołków przez macierze transformacji oraz używa wbudowanej zmiennej @builtin(front_facing) do dynamicznego dwustronnego nakładania kolorów.
* **state.rs** Główne serce aplikacji i zarządca biblioteki graficznej. Inicjalizuje kontekst wgpu, buffory, potok renderowania i zarządza logiką klatek animacji (w tym obliczaniem macierzy transformacji dla ruchu).
* **vertices.rs** Zawiera stałe geometryczne — współrzędne położenia wierzchołków oraz ich kolory zadeklarowane dla obu części zadań. Ponadto przechowuje odpowiednie zbiory indeksów służące do poprawnej triangulacji figur.

### Rozważane warianty i napotkane problemy:
1. Problem triangulacji trójkąta z dziurą (wybór pomiędzy 6 a 9 trójkątami)
Aby wyrenderować trójkąt A z wewnętrznym, pustym wycięciem w wariancie b, należało go poddać triangulacji. Ze względu na to, że układ graficzny renderuje jedynie pełne trójkąty, pojawił się problem optymalnego podziału powierzchni obręczy pomiędzy krawędzią zewnętrzną a wewnętrzną. Rozważano podział na 9 trójkątów, co mogłoby wyniknąć z dodania dodatkowych wierzchołków pomocniczych lub mniej optymalnego łączenia punktów. Ostatecznie postawiono na minimalizm i podzielono wycięty kształt na dokładnie 6 trójkątów (powstałych z podziału trzech czworokątów wokół otworu). Udane rozwiązanie wymagało manualnego zdefiniowania tablicy 18 wskaźników łączących odpowiednie wierzchołki, co przełożyło się na mniejsze obciążenie bufora indeksów i większą wydajność.
2. Problem wyboru formatu wektora pozycji (3- vs 4-elementowe typy)
Kolejnym napotkanym problemem była decyzja projektowa dotycząca struktury danych dla atrybutów wierzchołka: użycia 3 elementów (x, y, z) czy 4 elementów (x, y, z, w). Chociaż geometria przestrzenna operuje na trzech wymiarach, wybrano typ 4-elementowy [f32; 4]. Implementacja takich wektorów była podyktowana wymogami programów kolorujacych zdefiniowanych w WGSL, które przyjmują zmienne typu vec4<f32>. Posiadanie czwartego komponentu ("w" - współrzędnej jednorodnej ustawionej na wartość 1.0) było kluczowe, aby móc poprawnie pomnożyć wektor pozycji przez macierz transformacji rzutowania perspektywicznego view_proj oraz macierz modelu model_matrix w vertex shaderze. Oprócz spełnienia wymagań matematyki transformacyjnej, ułatwiło to także kwestię wyrównania pamięci (memory alignment) przesyłanej z Rusta prosto do API.

---

## II. Instrukcja uruchomienia

### Gotowe pliki wykonywalne:
W folderze głównym znajdują się skompilowane statycznie pliki `.exe` oraz `.png` i `.gif`:
1.  **`Zad1.exe || Zad1.png`** – Wizualizacja statyczna Części 1(b).
2.  **`Zad2.exe || Zad2.gif`** – Pełna animacja (180 klatek) Części 2(b).
3.  **`Zad2_36-45_frames.exe || Zad2_36-45_frames.gif`** – Skrócona wersja animacji (klatki 36-45) dla celów szybkiej weryfikacji.

**Sterowanie:**
* `Spacja` lub `Esc` – wyjście z aplikacji.
* Aby zmienić wyświetlane zadanie, należy zmienić wartość zmiennej $czy_zad1$ na $true$ w pliku `main.rs`.

### Kompilacja ze źródeł:
Wymagane środowisko Rust (https://rust-lang.org/tools/install/).
```bash
# Uruchomienie domyślne
cargo run

# Lub wersja produkcyjna
cargo run --release
# C++ Design Patterns

## Creational Patterns

### Singleton Pattern
```cpp
#include <memory>
#include <mutex>

class Singleton {
private:
    static std::unique_ptr<Singleton> instance;
    static std::mutex mutex;
    
    Singleton() = default;
    
public:
    // Delete copy constructor and assignment operator
    Singleton(const Singleton&) = delete;
    Singleton& operator=(const Singleton&) = delete;
    
    static Singleton& getInstance() {
        std::lock_guard<std::mutex> lock(mutex);
        if (!instance) {
            instance = std::unique_ptr<Singleton>(new Singleton());
        }
        return *instance;
    }
    
    void doSomething() {
        std::cout << "Singleton doing something" << std::endl;
    }
};

// Initialize static members
std::unique_ptr<Singleton> Singleton::instance = nullptr;
std::mutex Singleton::mutex;

int main() {
    Singleton& singleton = Singleton::getInstance();
    singleton.doSomething();
    return 0;
}
```

### Factory Pattern
```cpp
#include <memory>
#include <string>
#include <iostream>

// Abstract Product
class Animal {
public:
    virtual ~Animal() = default;
    virtual void makeSound() const = 0;
};

// Concrete Products
class Dog : public Animal {
public:
    void makeSound() const override {
        std::cout << "Woof!" << std::endl;
    }
};

class Cat : public Animal {
public:
    void makeSound() const override {
        std::cout << "Meow!" << std::endl;
    }
};

// Factory
class AnimalFactory {
public:
    enum class AnimalType { DOG, CAT };
    
    static std::unique_ptr<Animal> createAnimal(AnimalType type) {
        switch (type) {
            case AnimalType::DOG:
                return std::make_unique<Dog>();
            case AnimalType::CAT:
                return std::make_unique<Cat>();
            default:
                return nullptr;
        }
    }
};

int main() {
    auto dog = AnimalFactory::createAnimal(AnimalFactory::AnimalType::DOG);
    auto cat = AnimalFactory::createAnimal(AnimalFactory::AnimalType::CAT);
    
    dog->makeSound();
    cat->makeSound();
    
    return 0;
}
```

### Abstract Factory Pattern
```cpp
#include <memory>
#include <iostream>

// Abstract Product A
class Button {
public:
    virtual ~Button() = default;
    virtual void paint() const = 0;
};

// Abstract Product B
class Checkbox {
public:
    virtual ~Checkbox() = default;
    virtual void paint() const = 0;
};

// Concrete Products for Windows
class WindowsButton : public Button {
public:
    void paint() const override {
        std::cout << "Windows Button" << std::endl;
    }
};

class WindowsCheckbox : public Checkbox {
public:
    void paint() const override {
        std::cout << "Windows Checkbox" << std::endl;
    }
};

// Concrete Products for macOS
class MacOSButton : public Button {
public:
    void paint() const override {
        std::cout << "macOS Button" << std::endl;
    }
};

class MacOSCheckbox : public Checkbox {
public:
    void paint() const override {
        std::cout << "macOS Checkbox" << std::endl;
    }
};

// Abstract Factory
class GUIFactory {
public:
    virtual ~GUIFactory() = default;
    virtual std::unique_ptr<Button> createButton() const = 0;
    virtual std::unique_ptr<Checkbox> createCheckbox() const = 0;
};

// Concrete Factories
class WindowsFactory : public GUIFactory {
public:
    std::unique_ptr<Button> createButton() const override {
        return std::make_unique<WindowsButton>();
    }
    
    std::unique_ptr<Checkbox> createCheckbox() const override {
        return std::make_unique<WindowsCheckbox>();
    }
};

class MacOSFactory : public GUIFactory {
public:
    std::unique_ptr<Button> createButton() const override {
        return std::make_unique<MacOSButton>();
    }
    
    std::unique_ptr<Checkbox> createCheckbox() const override {
        return std::make_unique<MacOSCheckbox>();
    }
};

// Client
class Application {
private:
    std::unique_ptr<Button> button;
    std::unique_ptr<Checkbox> checkbox;
    
public:
    Application(std::unique_ptr<GUIFactory> factory) {
        button = factory->createButton();
        checkbox = factory->createCheckbox();
    }
    
    void createUI() {
        button->paint();
        checkbox->paint();
    }
};

int main() {
    auto windows_factory = std::make_unique<WindowsFactory>();
    Application windows_app(std::move(windows_factory));
    windows_app.createUI();
    
    auto macos_factory = std::make_unique<MacOSFactory>();
    Application macos_app(std::move(macos_factory));
    macos_app.createUI();
    
    return 0;
}
```

### Builder Pattern
```cpp
#include <string>
#include <vector>
#include <iostream>

class Computer {
private:
    std::string cpu;
    std::string gpu;
    int ram;
    int storage;
    
public:
    void setCPU(const std::string& cpu) { this->cpu = cpu; }
    void setGPU(const std::string& gpu) { this->gpu = gpu; }
    void setRAM(int ram) { this->ram = ram; }
    void setStorage(int storage) { this->storage = storage; }
    
    void display() const {
        std::cout << "Computer Specifications:\n";
        std::cout << "CPU: " << cpu << "\n";
        std::cout << "GPU: " << gpu << "\n";
        std::cout << "RAM: " << ram << "GB\n";
        std::cout << "Storage: " << storage << "GB\n";
    }
};

// Abstract Builder
class ComputerBuilder {
protected:
    Computer computer;
    
public:
    virtual ~ComputerBuilder() = default;
    virtual void buildCPU() = 0;
    virtual void buildGPU() = 0;
    virtual void buildRAM() = 0;
    virtual void buildStorage() = 0;
    
    Computer getComputer() { return computer; }
};

// Concrete Builder
class GamingComputerBuilder : public ComputerBuilder {
public:
    void buildCPU() override {
        computer.setCPU("Intel i9-12900K");
    }
    
    void buildGPU() override {
        computer.setGPU("NVIDIA RTX 4090");
    }
    
    void buildRAM() override {
        computer.setRAM(32);
    }
    
    void buildStorage() override {
        computer.setStorage(2000);
    }
};

class OfficeComputerBuilder : public ComputerBuilder {
public:
    void buildCPU() override {
        computer.setCPU("Intel i5-12400");
    }
    
    void buildGPU() override {
        computer.setGPU("Intel UHD Graphics");
    }
    
    void buildRAM() override {
        computer.setRAM(16);
    }
    
    void buildStorage() override {
        computer.setStorage(512);
    }
};

// Director
class ComputerDirector {
public:
    void construct(ComputerBuilder& builder) {
        builder.buildCPU();
        builder.buildGPU();
        builder.buildRAM();
        builder.buildStorage();
    }
};

int main() {
    ComputerDirector director;
    
    GamingComputerBuilder gaming_builder;
    director.construct(gaming_builder);
    Computer gaming_pc = gaming_builder.getComputer();
    gaming_pc.display();
    
    OfficeComputerBuilder office_builder;
    director.construct(office_builder);
    Computer office_pc = office_builder.getComputer();
    office_pc.display();
    
    return 0;
}
```

## Structural Patterns

### Adapter Pattern
```cpp
#include <iostream>

// Target interface
class MediaPlayer {
public:
    virtual ~MediaPlayer() = default;
    virtual void play(const std::string& audioType, const std::string& fileName) = 0;
};

// Adaptee
class AdvancedMediaPlayer {
public:
    virtual ~AdvancedMediaPlayer() = default;
    virtual void playVlc(const std::string& fileName) = 0;
    virtual void playMp4(const std::string& fileName) = 0;
};

class VlcPlayer : public AdvancedMediaPlayer {
public:
    void playVlc(const std::string& fileName) override {
        std::cout << "Playing vlc file: " << fileName << std::endl;
    }
    
    void playMp4(const std::string& fileName) override {}
};

class Mp4Player : public AdvancedMediaPlayer {
public:
    void playVlc(const std::string& fileName) override {}
    
    void playMp4(const std::string& fileName) override {
        std::cout << "Playing mp4 file: " << fileName << std::endl;
    }
};

// Adapter
class MediaAdapter : public MediaPlayer {
private:
    std::unique_ptr<AdvancedMediaPlayer> advancedMusicPlayer;
    
public:
    MediaAdapter(const std::string& audioType) {
        if (audioType == "vlc") {
            advancedMusicPlayer = std::make_unique<VlcPlayer>();
        } else if (audioType == "mp4") {
            advancedMusicPlayer = std::make_unique<Mp4Player>();
        }
    }
    
    void play(const std::string& audioType, const std::string& fileName) override {
        if (audioType == "vlc") {
            advancedMusicPlayer->playVlc(fileName);
        } else if (audioType == "mp4") {
            advancedMusicPlayer->playMp4(fileName);
        }
    }
};

// Client
class AudioPlayer : public MediaPlayer {
private:
    std::unique_ptr<MediaAdapter> mediaAdapter;
    
public:
    void play(const std::string& audioType, const std::string& fileName) override {
        if (audioType == "mp3") {
            std::cout << "Playing mp3 file: " << fileName << std::endl;
        } else if (audioType == "vlc" || audioType == "mp4") {
            mediaAdapter = std::make_unique<MediaAdapter>(audioType);
            mediaAdapter->play(audioType, fileName);
        } else {
            std::cout << "Invalid media. " << audioType << " format not supported" << std::endl;
        }
    }
};

int main() {
    AudioPlayer player;
    
    player.play("mp3", "song.mp3");
    player.play("mp4", "video.mp4");
    player.play("vlc", "movie.vlc");
    player.play("avi", "movie.avi");
    
    return 0;
}
```

### Decorator Pattern
```cpp
#include <iostream>
#include <string>

// Component
class Coffee {
public:
    virtual ~Coffee() = default;
    virtual double cost() const = 0;
    virtual std::string description() const = 0;
};

// Concrete Component
class SimpleCoffee : public Coffee {
public:
    double cost() const override {
        return 2.0;
    }
    
    std::string description() const override {
        return "Simple coffee";
    }
};

// Decorator
class CoffeeDecorator : public Coffee {
private:
    std::unique_ptr<Coffee> coffee;
    
protected:
    CoffeeDecorator(std::unique_ptr<Coffee> coffee) : coffee(std::move(coffee)) {}
    
public:
    std::string description() const override {
        return coffee->description();
    }
};

// Concrete Decorators
class MilkDecorator : public CoffeeDecorator {
public:
    MilkDecorator(std::unique_ptr<Coffee> coffee) : CoffeeDecorator(std::move(coffee)) {}
    
    double cost() const override {
        return coffee->cost() + 0.5;
    }
    
    std::string description() const override {
        return coffee->description() + ", milk";
    }
};

class SugarDecorator : public CoffeeDecorator {
public:
    SugarDecorator(std::unique_ptr<Coffee> coffee) : CoffeeDecorator(std::move(coffee)) {}
    
    double cost() const override {
        return coffee->cost() + 0.2;
    }
    
    std::string description() const override {
        return coffee->description() + ", sugar";
    }
};

class VanillaDecorator : public CoffeeDecorator {
public:
    VanillaDecorator(std::unique_ptr<Coffee> coffee) : CoffeeDecorator(std::move(coffee)) {}
    
    double cost() const override {
        return coffee->cost() + 0.7;
    }
    
    std::string description() const override {
        return coffee->description() + ", vanilla";
    }
};

int main() {
    auto coffee = std::make_unique<SimpleCoffee>();
    std::cout << coffee->description() << " $" << coffee->cost() << std::endl;
    
    coffee = std::make_unique<MilkDecorator>(std::move(coffee));
    std::cout << coffee->description() << " $" << coffee->cost() << std::endl;
    
    coffee = std::make_unique<SugarDecorator>(std::move(coffee));
    std::cout << coffee->description() << " $" << coffee->cost() << std::endl;
    
    coffee = std::make_unique<VanillaDecorator>(std::move(coffee));
    std::cout << coffee->description() << " $" << coffee->cost() << std::endl;
    
    return 0;
}
```

### Proxy Pattern
```cpp
#include <iostream>
#include <memory>

// Subject
class Image {
public:
    virtual ~Image() = default;
    virtual void display() = 0;
};

// Real Subject
class RealImage : public Image {
private:
    std::string filename;
    
public:
    RealImage(const std::string& filename) : filename(filename) {
        loadFromDisk();
    }
    
    void display() override {
        std::cout << "Displaying " << filename << std::endl;
    }
    
private:
    void loadFromDisk() {
        std::cout << "Loading " << filename << " from disk" << std::endl;
    }
};

// Proxy
class ProxyImage : public Image {
private:
    std::unique_ptr<RealImage> realImage;
    std::string filename;
    
public:
    ProxyImage(const std::string& filename) : filename(filename) {}
    
    void display() override {
        if (!realImage) {
            realImage = std::make_unique<RealImage>(filename);
        }
        realImage->display();
    }
};

// Client
class ImageViewer {
public:
    void viewImage(const std::string& filename) {
        auto image = std::make_unique<ProxyImage>(filename);
        image->display();
        std::cout << "Image displayed" << std::endl;
    }
};

int main() {
    ImageViewer viewer;
    viewer.viewImage("image1.jpg");
    viewer.viewImage("image2.jpg");
    
    return 0;
}
```

## Behavioral Patterns

### Observer Pattern
```cpp
#include <vector>
#include <memory>
#include <string>
#include <iostream>

// Observer
class Observer {
public:
    virtual ~Observer() = default;
    virtual void update(const std::string& message) = 0;
};

// Subject
class Subject {
private:
    std::vector<std::weak_ptr<Observer>> observers;
    
public:
    void attach(std::shared_ptr<Observer> observer) {
        observers.push_back(observer);
    }
    
    void detach(std::shared_ptr<Observer> observer) {
        observers.erase(
            std::remove_if(observers.begin(), observers.end(),
                [&](const std::weak_ptr<Observer>& weak_obs) {
                    return weak_obs.lock() == observer;
                }),
            observers.end()
        );
    }
    
    void notify(const std::string& message) {
        for (auto it = observers.begin(); it != observers.end();) {
            if (auto obs = it->lock()) {
                obs->update(message);
                ++it;
            } else {
                // Remove expired weak_ptr
                it = observers.erase(it);
            }
        }
    }
};

// Concrete Observer
class EmailNotificationObserver : public Observer {
private:
    std::string email;
    
public:
    EmailNotificationObserver(const std::string& email) : email(email) {}
    
    void update(const std::string& message) override {
        std::cout << "Email to " << email << ": " << message << std::endl;
    }
};

class SMSNotificationObserver : public Observer {
private:
    std::string phoneNumber;
    
public:
    SMSNotificationObserver(const std::string& phone) : phoneNumber(phone) {}
    
    void update(const std::string& message) override {
        std::cout << "SMS to " << phoneNumber << ": " << message << std::endl;
    }
};

// Concrete Subject
class NewsAgency : public Subject {
private:
    std::string latestNews;
    
public:
    void setNews(const std::string& news) {
        latestNews = news;
        notify("Breaking News: " + news);
    }
};

int main() {
    NewsAgency newsAgency;
    
    auto email_obs1 = std::make_shared<EmailNotificationObserver>("user1@example.com");
    auto email_obs2 = std::make_shared<EmailNotificationObserver>("user2@example.com");
    auto sms_obs = std::make_shared<SMSNotificationObserver>("123-456-7890");
    
    newsAgency.attach(email_obs1);
    newsAgency.attach(email_obs2);
    newsAgency.attach(sms_obs);
    
    newsAgency.setNews("New C++ Design Patterns Book Released!");
    
    // Detach one observer
    newsAgency.detach(email_obs2);
    
    newsAgency.setNews("Updated with Advanced Patterns Chapter!");
    
    return 0;
}
```

### Strategy Pattern
```cpp
#include <memory>
#include <vector>
#include <algorithm>
#include <iostream>

// Strategy Interface
class SortStrategy {
public:
    virtual ~SortStrategy() = default;
    virtual void sort(std::vector<int>& data) = 0;
};

// Concrete Strategies
class BubbleSortStrategy : public SortStrategy {
public:
    void sort(std::vector<int>& data) override {
        std::cout << "Using Bubble Sort" << std::endl;
        int n = data.size();
        for (int i = 0; i < n - 1; ++i) {
            for (int j = 0; j < n - i - 1; ++j) {
                if (data[j] > data[j + 1]) {
                    std::swap(data[j], data[j + 1]);
                }
            }
        }
    }
};

class QuickSortStrategy : public SortStrategy {
private:
    void quickSort(std::vector<int>& data, int low, int high) {
        if (low < high) {
            int pi = partition(data, low, high);
            quickSort(data, low, pi - 1);
            quickSort(data, pi + 1, high);
        }
    }
    
    int partition(std::vector<int>& data, int low, int high) {
        int pivot = data[high];
        int i = low - 1;
        
        for (int j = low; j < high; ++j) {
            if (data[j] < pivot) {
                ++i;
                std::swap(data[i], data[j]);
            }
        }
        std::swap(data[i + 1], data[high]);
        return i + 1;
    }
    
public:
    void sort(std::vector<int>& data) override {
        std::cout << "Using Quick Sort" << std::endl;
        if (!data.empty()) {
            quickSort(data, 0, data.size() - 1);
        }
    }
};

class MergeSortStrategy : public SortStrategy {
private:
    void merge(std::vector<int>& data, int left, int mid, int right) {
        int n1 = mid - left + 1;
        int n2 = right - mid;
        
        std::vector<int> L(n1), R(n2);
        
        for (int i = 0; i < n1; ++i) L[i] = data[left + i];
        for (int j = 0; j < n2; ++j) R[j] = data[mid + 1 + j];
        
        int i = 0, j = 0, k = left;
        
        while (i < n1 && j < n2) {
            if (L[i] <= R[j]) {
                data[k++] = L[i++];
            } else {
                data[k++] = R[j++];
            }
        }
        
        while (i < n1) data[k++] = L[i++];
        while (j < n2) data[k++] = R[j++];
    }
    
    void mergeSort(std::vector<int>& data, int left, int right) {
        if (left < right) {
            int mid = left + (right - left) / 2;
            mergeSort(data, left, mid);
            mergeSort(data, mid + 1, right);
            merge(data, left, mid, right);
        }
    }
    
public:
    void sort(std::vector<int>& data) override {
        std::cout << "Using Merge Sort" << std::endl;
        if (!data.empty()) {
            mergeSort(data, 0, data.size() - 1);
        }
    }
};

// Context
class Sorter {
private:
    std::unique_ptr<SortStrategy> strategy;
    
public:
    void setStrategy(std::unique_ptr<SortStrategy> newStrategy) {
        strategy = std::move(newStrategy);
    }
    
    void performSort(std::vector<int>& data) {
        if (strategy) {
            strategy->sort(data);
        }
    }
};

int main() {
    Sorter sorter;
    std::vector<int> data = {64, 34, 25, 12, 22, 11, 90};
    
    std::cout << "Original: ";
    for (int num : data) std::cout << num << " ";
    std::cout << std::endl;
    
    sorter.setStrategy(std::make_unique<BubbleSortStrategy>());
    sorter.performSort(data);
    
    std::cout << "Sorted: ";
    for (int num : data) std::cout << num << " ";
    std::cout << std::endl;
    
    // Reset data and try different strategy
    data = {64, 34, 25, 12, 22, 11, 90};
    sorter.setStrategy(std::make_unique<QuickSortStrategy>());
    sorter.performSort(data);
    
    std::cout << "Sorted: ";
    for (int num : data) std::cout << num << " ";
    std::cout << std::endl;
    
    return 0;
}
```

### Command Pattern
```cpp
#include <vector>
#include <memory>
#include <iostream>

// Command
class Command {
public:
    virtual ~Command() = default;
    virtual void execute() = 0;
    virtual void undo() = 0;
};

// Receiver
class Light {
public:
    void turnOn() {
        std::cout << "Light is ON" << std::endl;
    }
    
    void turnOff() {
        std::cout << "Light is OFF" << std::endl;
    }
};

// Concrete Commands
class LightOnCommand : public Command {
private:
    Light& light;
    
public:
    LightOnCommand(Light& light) : light(light) {}
    
    void execute() override {
        light.turnOn();
    }
    
    void undo() override {
        light.turnOff();
    }
};

class LightOffCommand : public Command {
private:
    Light& light;
    
public:
    LightOffCommand(Light& light) : light(light) {}
    
    void execute() override {
        light.turnOff();
    }
    
    void undo() override {
        light.turnOn();
    }
};

// Invoker
class RemoteControl {
private:
    std::vector<std::unique_ptr<Command>> commands;
    std::vector<std::unique_ptr<Command>> undoStack;
    
public:
    void executeCommand(std::unique_ptr<Command> command) {
        command->execute();
        commands.push_back(std::move(command));
        undoStack.clear();  // Clear undo stack when new command is executed
    }
    
    void undo() {
        if (!commands.empty()) {
            auto lastCommand = std::move(commands.back());
            commands.pop_back();
            lastCommand->undo();
            undoStack.push_back(std::move(lastCommand));
        }
    }
    
    void redo() {
        if (!undoStack.empty()) {
            auto lastUndone = std::move(undoStack.back());
            undoStack.pop_back();
            lastUndone->execute();
            commands.push_back(std::move(lastUndone));
        }
    }
};

int main() {
    Light livingRoomLight;
    RemoteControl remote;
    
    // Execute commands
    remote.executeCommand(std::make_unique<LightOnCommand>(livingRoomLight));
    remote.executeCommand(std::make_unique<LightOffCommand>(livingRoomLight));
    
    // Undo last command
    std::cout << "Undoing last command:" << std::endl;
    remote.undo();
    
    // Redo command
    std::cout << "Redoing command:" << std::endl;
    remote.redo();
    
    return 0;
}
```

## Modern C++ Pattern Implementations

### Using Modern C++ Features
```cpp
#include <memory>
#include <functional>
#include <variant>
#include <iostream>

// Modern Observer using std::function
class ModernSubject {
private:
    std::vector<std::function<void(const std::string&)>> observers;
    
public:
    void attach(std::function<void(const std::string&)> observer) {
        observers.push_back(observer);
    }
    
    void notify(const std::string& message) {
        for (const auto& observer : observers) {
            observer(message);
        }
    }
};

// Modern Strategy using std::function and lambda
class ModernSorter {
private:
    std::function<void(std::vector<int>&)> strategy;
    
public:
    void setStrategy(std::function<void(std::vector<int>&)> newStrategy) {
        strategy = newStrategy;
    }
    
    void sort(std::vector<int>& data) {
        if (strategy) {
            strategy(data);
        }
    }
};

// Modern Command using std::variant
class ModernCommand {
public:
    using CommandVariant = std::variant<
        std::function<void()>,  // Simple command
        std::function<void(std::string&)>,  // Command with parameter
        std::function<void()>  // Complex command
    >;
    
private:
    CommandVariant command;
    
public:
    template<typename T>
    ModernCommand(T cmd) : command(std::move(cmd)) {}
    
    void execute() {
        std::visit([](auto&& cmd) {
            if constexpr (std::is_invocable_v<decltype(cmd)>) {
                if constexpr (std::is_invocable_v<decltype(cmd), std::string&>) {
                    std::string dummy;
                    cmd(dummy);
                } else {
                    cmd();
                }
            }
        }, command);
    }
};

int main() {
    // Modern Observer
    ModernSubject subject;
    
    subject.attach([](const std::string& msg) {
        std::cout << "Observer 1: " << msg << std::endl;
    });
    
    subject.attach([](const std::string& msg) {
        std::cout << "Observer 2: " << msg << std::endl;
    });
    
    subject.notify("Hello from Modern Observer!");
    
    // Modern Strategy
    ModernSorter sorter;
    std::vector<int> data = {5, 2, 8, 1, 9};
    
    sorter.setStrategy([](std::vector<int>& vec) {
        std::cout << "Using Lambda Sort Strategy" << std::endl;
        std::sort(vec.begin(), vec.end());
    });
    
    sorter.sort(data);
    
    for (int num : data) std::cout << num << " ";
    std::cout << std::endl;
    
    return 0;
}
```

## Best Practices Summary
- Use smart pointers for automatic memory management
- Leverage modern C++ features (auto, lambda, concepts, etc.)
- Prefer composition over inheritance when possible
- Use interfaces (abstract classes) for loose coupling
- Implement RAII for resource management
- Use const correctness throughout your code
- Consider thread safety in multi-threaded environments
- Use appropriate design patterns for the problem domain
- Keep patterns simple and don't over-engineer
- Document pattern usage and intent
- Test pattern implementations thoroughly
- Consider performance implications of patterns
- Use modern C++ alternatives when available (e.g., std::variant over Visitor pattern)
